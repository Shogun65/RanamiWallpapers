#include "Engine.h"
#include <cstdlib>
#include <string>
#include <fstream>

void SavePath(const std::string &filepath)
{
	std::ofstream wallpaper_file_txt("wallpaper_path.txt", std::ios::trunc);

	if(!wallpaper_file_txt) return;

	wallpaper_file_txt << filepath;

}

std::string LoadPath()
{

	std::ifstream wallpaper_file_txt("wallpaper_path.txt");

	std::string file_path = "";

	if(!wallpaper_file_txt) return file_path;

	std::getline(wallpaper_file_txt, file_path);

	printf("file_path: %ws", file_path);

	return file_path;
}

int WINAPI wWinMain(
	HINSTANCE hInstance, HINSTANCE ignore, LPWSTR pCmdLine, int nCmdShow)
{
	// ignore this
	SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
	
	const char* fileparth = "C:/Users/gmy87/Downloads/female-endministrator.3840x2160.mp4";
	int buffersize = 3;

	Engine engine;

	wchar_t* fileflage = wcsstr(pCmdLine, L"-f-");

	printf("fileFlage: %ws\n", fileflage);

	if (fileflage)
	{
		// one extry for space -f- c the space between f-_c <- this if you can understand
		// what iam saying
		fileflage += 4;
		 
		static char filebuffer[512];

		size_t converted = 0;

		wcstombs_s(&converted, filebuffer, sizeof(filebuffer), fileflage, _TRUNCATE);

		fileparth = filebuffer;
	}


	engine.MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(hInstance,
		fileparth, buffersize);
	
	return EXIT_SUCCESS;
}