pub(crate) mod thumbnail_cache {

    use slint::Image;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::{Command, Stdio};

    const THUMBNAIL_CACHE_DIR: &str = "cache-wallpaper-thumbnails";
    const THUMBNAIL_EXTENSION: &str = "jpg";
    const THUMBNAIL_WIDTH: &str = "640";
    const THUMBNAIL_HEIGHT: &str = "360";
    const THUMBNAIL_TIMESTAMP: &str = "00:00:01";

    pub fn load_or_create_thumbnail(video_path: &str) -> Result<Image, String> {
        // Each card asks for its preview through this single entry point:
        // build the cache path, regenerate only when needed, then load it for Slint.
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

        Image::load_from_path(&thumbnail_path).map_err(|_| {
            format!(
                "Slint could not load thumbnail image: {}",
                thumbnail_path.display()
            )
        })
    }

    fn thumbnail_path_for_video(video_path: &Path) -> Result<PathBuf, String> {
        // Keep generated previews in a local cache folder instead of polluting the repo root.
        let cache_dir = env::current_dir()
            .map_err(|err| format!("Could not read current directory: {err}"))?
            .join(THUMBNAIL_CACHE_DIR);

        fs::create_dir_all(&cache_dir).map_err(|err| {
            format!(
                "Could not create thumbnail cache directory {}: {err}",
                cache_dir.display()
            )
        })?;

        // The cached file name uses both the video name and a hash of the full path,
        // so two different folders can still have their own "video.mp4" safely.
        Ok(cache_dir.join(format!(
            "{}_{}.{}",
            sanitized_video_stem(video_path),
            stable_path_hash(&video_path.to_string_lossy()),
            THUMBNAIL_EXTENSION
        )))
    }

    fn generate_thumbnail(video_path: &Path, thumbnail_path: &Path) -> Result<(), String> {
        let ffmpeg_command = ffmpeg_command();

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

    fn ffmpeg_command() -> PathBuf {
        ffmpeg_candidates()
            .into_iter()
            .find(|candidate| candidate.exists())
            // If none of our guessed paths exists, fall back to PATH lookup.
            .unwrap_or_else(|| PathBuf::from("ffmpeg.exe"))
    }

    fn ffmpeg_candidates() -> Vec<PathBuf> {
        let mut candidates = Vec::new();

        // First try locations near the running GUI executable.
        if let Ok(current_exe) = env::current_exe() {
            if let Some(exe_dir) = current_exe.parent() {
                candidates.push(exe_dir.join("ffmpeg.exe"));

                if let Some(target_dir) = exe_dir.parent() {
                    candidates.push(target_dir.join("release").join("ffmpeg.exe"));
                    candidates.push(target_dir.join("debug").join("ffmpeg.exe"));
                }
            }
        }

        // Then try common dev paths in this repo, including rust/target/release where you placed ffmpeg.exe.
        if let Ok(current_dir) = env::current_dir() {
            candidates.push(current_dir.join("ffmpeg.exe"));
            candidates.push(
                current_dir
                    .join("target")
                    .join("release")
                    .join("ffmpeg.exe"),
            );
            candidates.push(
                current_dir
                    .join("rust")
                    .join("target")
                    .join("release")
                    .join("ffmpeg.exe"),
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
