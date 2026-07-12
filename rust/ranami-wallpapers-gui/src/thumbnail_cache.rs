pub(crate) mod thumbnail_cache {
    use std::collections::HashSet;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};
    use shared::save_path_and_settings::*;

    pub fn load_or_create_thumbnail_path(video_path: &str) -> Result<PathBuf, String> {
        // Each card asks for its preview through this single entry point:
        // build the cache path and regenerate only when needed.
        let video_path = Path::new(video_path);

        if !video_path.exists() {
            return Err(format!(
                "Wallpaper video was not found: {}",
                video_path.display()
            ));
        }

        let thumbnail_path = thumbnail_path_for_video(video_path)?;

        if needs_regeneration(video_path, &thumbnail_path)? {
            generate_thumbnail(video_path, &thumbnail_path)?;
        }

        Ok(thumbnail_path)
    }

    pub fn cleanup_stale_thumbnails(video_paths: &[String]) -> Result<(), String> {
        let cache_dir = thumbnail_cache_dir()?;

        // Keep only thumbnails that still belong to saved wallpapers.
        let expected_paths: HashSet<PathBuf> = video_paths
            .iter()
            .map(|video_path| thumbnail_path_for_video(Path::new(video_path)))
            .collect::<Result<_, _>>()?;

        for entry in fs::read_dir(&cache_dir).map_err(|err| {
            format!(
                "Could not read thumbnail cache directory {}: {err}",
                cache_dir.display()
            )
        })? {
            let entry = entry.map_err(|err| {
                format!(
                    "Could not inspect thumbnail cache directory {}: {err}",
                    cache_dir.display()
                )
            })?;
            let thumbnail_path = entry.path();

            if !thumbnail_path.is_file() {
                continue;
            }

            let is_thumbnail = thumbnail_path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case(THUMBNAIL_EXTENSION));

            if is_thumbnail && !expected_paths.contains(&thumbnail_path) {
                fs::remove_file(&thumbnail_path).map_err(|err| {
                    format!(
                        "Could not remove stale thumbnail {}: {err}",
                        thumbnail_path.display()
                    )
                })?;
            }
        }

        Ok(())
    }

    fn thumbnail_path_for_video(video_path: &Path) -> Result<PathBuf, String> {
        // Keep generated previews in a local cache folder instead of polluting the repo root.
        let cache_dir = thumbnail_cache_dir()?;

        // The cached file name uses both the video name and a hash of the full path,
        // so two different folders can still have their own "video.mp4" safely.
        Ok(cache_dir.join(format!(
            "{}_{}.{}",
            sanitized_video_stem(video_path),
            stable_path_hash(&video_path.to_string_lossy()),
            THUMBNAIL_EXTENSION
        )))
    }

    fn thumbnail_cache_dir() -> Result<PathBuf, String> {
        let cache_dir = env::current_dir()
            .map_err(|err| format!("Could not read current directory: {err}"))?
            .join(THUMBNAIL_CACHE_DIR);

        fs::create_dir_all(&cache_dir).map_err(|err| {
            format!(
                "Could not create thumbnail cache directory {}: {err}",
                cache_dir.display()
            )
        })?;

        Ok(cache_dir)
    }

    fn generate_thumbnail(video_path: &Path, thumbnail_path: &Path) -> Result<(), String> {
        let ffmpeg_command = ffmpeg_command_path();

        // Grab one frame around 1 second in and crop it to a card-friendly preview size.
        let status = Command::new(&ffmpeg_command)
            .arg("-y")
            .arg("-ss")
            .arg(THUMBNAIL_TIMESTAMP)
            .arg("-i")
            .arg(video_path)
            .arg("-frames:v")
            .arg("1")
            .arg("-vf")
            .arg(format!(
                "scale={THUMBNAIL_WIDTH}:{THUMBNAIL_HEIGHT}:force_original_aspect_ratio=increase,crop={THUMBNAIL_WIDTH}:{THUMBNAIL_HEIGHT}"
            ))
            .arg("-q:v")
            .arg("3")
            .arg(thumbnail_path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|err| {
                format!(
                    "Could not start ffmpeg command '{}' for {}: {err}",
                    ffmpeg_command.display(),
                    video_path.display()
                )
            })?;

        if !status.success() {
            return Err(format!(
                "ffmpeg failed while generating thumbnail for {}",
                video_path.display()
            ));
        }

        if !thumbnail_path.exists() {
            return Err(format!(
                "ffmpeg reported success but no thumbnail was created at {}",
                thumbnail_path.display()
            ));
        }

        Ok(())
    }

    fn needs_regeneration(video_path: &Path, thumbnail_path: &Path) -> Result<bool, String> {
        // Rebuild only when the cache is missing or older than the source video.
        if !thumbnail_path.exists() {
            return Ok(true);
        }

        let video_metadata = fs::metadata(video_path).map_err(|err| {
            format!(
                "Could not read video metadata for {}: {err}",
                video_path.display()
            )
        })?;
        let thumbnail_metadata = fs::metadata(thumbnail_path).map_err(|err| {
            format!(
                "Could not read thumbnail metadata for {}: {err}",
                thumbnail_path.display()
            )
        })?;

        let video_modified = match video_metadata.modified() {
            Ok(video_modified) => video_modified,
            Err(_) => return Ok(false),
        };

        let thumbnail_modified = match thumbnail_metadata.modified() {
            Ok(thumbnail_modified) => thumbnail_modified,
            Err(_) => return Ok(true),
        };

        Ok(thumbnail_modified < video_modified)
    }

    pub fn ffmpeg_command_path() -> PathBuf {
        command_path("ffmpeg.exe")
    }

    pub fn ffprobe_command_path() -> PathBuf {
        command_path("ffprobe.exe")
    }

    fn command_path(binary_name: &str) -> PathBuf {
        command_candidates(binary_name)
            .into_iter()
            .find(|candidate| candidate.exists())
            // If none of our guessed paths exists, fall back to PATH lookup.
            .unwrap_or_else(|| PathBuf::from(binary_name))
    }

    fn command_candidates(binary_name: &str) -> Vec<PathBuf> {
        // Search a few practical locations before relying on PATH, because shipped builds
        // often keep ffmpeg / ffprobe beside the app or inside the target release folder.
        let mut candidates = Vec::new();

        // First try locations near the running GUI executable.
        if let Ok(current_exe) = env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                candidates.push(exe_dir.join(binary_name));

                if let Some(target_dir) = exe_dir.parent() {
                    candidates.push(target_dir.join("release").join(binary_name));
                    candidates.push(target_dir.join("debug").join(binary_name));
                }
            }
        }

        // Then try common dev paths in this repo, including rust/target/release where you placed the binaries.
        if let Ok(current_dir) = env::current_dir() {
            candidates.push(current_dir.join(binary_name));
            candidates.push(current_dir.join("target").join("release").join(binary_name));
            candidates.push(
                current_dir
                    .join("rust")
                    .join("target")
                    .join("release")
                    .join(binary_name),
            );
        }

        candidates
    }

    fn sanitized_video_stem(video_path: &Path) -> String {
        // Clean the file name so the cache file stays Windows-friendly.
        let file_stem = video_path
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or("wallpaper");

        let mut sanitized = String::with_capacity(file_stem.len());

        for character in file_stem.chars() {
            if character.is_ascii_alphanumeric() {
                sanitized.push(character.to_ascii_lowercase());
            } else if character == '-' || character == '_' {
                sanitized.push(character);
            } else {
                sanitized.push('-');
            }
        }

        let sanitized = sanitized.trim_matches('-');

        if sanitized.is_empty() {
            "wallpaper".to_string()
        } else {
            sanitized.chars().take(48).collect()
        }
    }

    fn stable_path_hash(path: &str) -> String {
        // A tiny stable hash is enough here; it just prevents thumbnail file name collisions.
        let mut hash: u64 = 0xcbf29ce484222325;

        for byte in path.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }

        format!("{hash:016x}")
    }
}
