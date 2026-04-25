#include "Engine.h"
#include <cstdlib>
#include <string>
#include <shobjidl.h>
#include <fstream>

#pragma comment(lib, "Ole32.lib")

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

	printf("file_path: %s\n", file_path.c_str());

	return file_path;
}

std::string PickVideoFile()
{
    HRESULT hr = CoInitializeEx(nullptr, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
    if (FAILED(hr))
        return "";

    IFileOpenDialog* pFileOpen = nullptr;

    hr = CoCreateInstance(
        CLSID_FileOpenDialog,
        nullptr,
        CLSCTX_ALL,
        IID_IFileOpenDialog,
        (void**)&pFileOpen
    );

    if (FAILED(hr))
    {
        CoUninitialize();
        return "";
    }

    COMDLG_FILTERSPEC fileTypes[] =
    {
        { L"Video Files", L"*.mp4;*.mkv;*.avi;*.mov;*.wmv" },
        { L"All Files", L"*.*" }
    };

    pFileOpen->SetFileTypes(2, fileTypes);
    pFileOpen->SetTitle(L"Choose a wallpaper video");

    hr = pFileOpen->Show(nullptr);

    if (SUCCEEDED(hr))
    {
        IShellItem* pItem = nullptr;
        hr = pFileOpen->GetResult(&pItem);

        if (SUCCEEDED(hr))
        {
            PWSTR widePath = nullptr;
            hr = pItem->GetDisplayName(SIGDN_FILESYSPATH, &widePath);

            if (SUCCEEDED(hr))
            {
                char pathBuffer[512];
                size_t converted = 0;
                wcstombs_s(&converted, pathBuffer, sizeof(pathBuffer), widePath, _TRUNCATE);

                std::string result = pathBuffer;

                CoTaskMemFree(widePath);
                pItem->Release();
                pFileOpen->Release();
                CoUninitialize();

                return result;
            }

            pItem->Release();
        }
    }

    pFileOpen->Release();
    CoUninitialize();
    return "";
}


int WINAPI wWinMain(
	HINSTANCE hInstance, HINSTANCE ignore, LPWSTR pCmdLine, int nCmdShow)
{
	// ignore this
	SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
	
	int buffersize = 3;

	Engine engine;

	std::string video_path = LoadPath();

	if(video_path.empty())
	{
		video_path = PickVideoFile();
		if(!video_path.empty())
		{
			SavePath(video_path);
		}
	}

	if(video_path.empty())
	{
		MessageBox(nullptr, L"Video Path not exist!", L"Error", MB_ICONERROR);
		return EXIT_FAILURE;
	}

	engine.MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(
		hInstance,
		video_path.c_str(), 
		buffersize);
	
	return EXIT_SUCCESS;
}
