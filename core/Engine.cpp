#include "Engine.h"

Engine::~Engine()
{
	_ffmpeg.ShutDownDecoder();
	if(_DecodeingLoop_Thread.joinable())
	{
		_DecodeingLoop_Thread.join();
	}
}

void Engine::CreateNormalWindowAndRun(HINSTANCE hInstance)
{
	_window.InitDebugConsole();
	_window.CreateMainWindow(hInstance);
	_window.ShowMainWindow();
	//_window.MessageLoopRun();
}

void Engine::CreateWindowOnWorkerWAndRun(HINSTANCE hInstance)
{
	_window.InitDebugConsole();
	_window.CreateMainWindow(hInstance);
	_workerW.SpawnWorkerW();
	_workerW.FindWorkerW();
	_window.AttachHwndToWorkerW(_workerW.GetWorkerW());
	_window.ShowMainWindow();
	//_window.MessageLoopRun();
}

void Engine::SeeWindowTree()
{
	_window.InitDebugConsole();
	_workerW.SpawnWorkerW();
	_workerW.PrintWindowThree();
}

void Engine::testDx()
{
	_window.InitDebugConsole();
	_dxdevice.CreateDeviceAndDeviceContext();
}

void Engine::testDXandSwapchin(HINSTANCE hInstance)
{
	
	_window.CreateMainWindow(hInstance);
	_dxdevice.CreateDeviceAndDeviceContext();

	_swapchin.CreateSwapChin1(
		_window.GetWindowHeight(),
		_window.GetWindowWidth(),
		_dxdevice.GetDevice());
}

void Engine::testDXandswapanddcomp(HINSTANCE hInstance)
{
	_window.InitDebugConsole();
	_window.CreateMainWindow(hInstance);
	_dxdevice.CreateDeviceAndDeviceContext();

	_swapchin.CreateSwapChin1(
		_window.GetWindowHeight(),
		_window.GetWindowWidth(),
		_dxdevice.GetDevice());
	_swapchin.CreateRTVForBackBuffer(_dxdevice.GetDevice(), _dxdevice.GetDeviceContext());
	_DComp.CreateDComp(_window.GetHWND(), _swapchin.GetSwapChin(), _dxdevice.GetDevice());

}

/*
*	Just my though ignore this:
*	So today is 25th march of 2026.. and i working on this project
*	because i want a wallpaper engine but wallpaper engien is not free and not
*	Expensive too but i cant Afford because my parent dont really give me money
*	for this kinda stuff. soo i start lookinh for other wallpaper engine options but
*	other are heavy like really.. some of take like 70 or 100% use of my intel IGPU
*	FYI i have a intel iris Xe.. and i want it too like take only 50 or 40% of use my
*	GPU okay.. so thats why i started this project. and i test the prototype and it
*	Working good i use everyday.. i set it to 'shell:startup' soo now it open as my laptop
*	open and also it was not easy i choose to use DX11 and FYI agian i have no idea
*	what a Graphic programming even is. and also by this project i learn so many new
*	thinks and iam glad that i started this project. i mainly started this project like
*	january 7 or 8th (2026) i forget but somethink like that thank you and sorry for my
*	bad english 😅
*/

void Engine::MakeWindowRunwhitWorkerWandRunDXandswapchin(HINSTANCE hInstance)
{
	_window.InitDebugConsole();
	_window.CreateMainWindow(hInstance);
	_workerW.SpawnWorkerW();
	_workerW.FindWorkerW();
	_window.AttachHwndToWorkerW(_workerW.GetWorkerW());
	_dxdevice.CreateDeviceAndDeviceContext();
	_swapchin.CreateSwapChin1(
		_window.GetWindowHeight(),
		_window.GetWindowWidth(),
		_dxdevice.GetDevice());
	_swapchin.CreateRTVForBackBuffer(_dxdevice.GetDevice(), _dxdevice.GetDeviceContext());
	_DComp.CreateDComp(_window.GetHWND(), _swapchin.GetSwapChin(), _dxdevice.GetDevice());
	_window.ShowMainWindow();
	
	_window.MessageLoopRun([&]() {_render.cleanscreen(_swapchin.GetRTVOfBackBuffer(), _swapchin.GetSwapChin(), _dxdevice.GetDeviceContext()); });
	
}

void Engine::MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(HINSTANCE hInstance, 
	const char* fileparth, int sizeofbuffer)
{
	_window.InitDebugConsole();
	_framequeue.init(sizeofbuffer);
	_framepool.init(_framequeue.GetSizeofBuffer()); //do this because sizeofbuffer need to be clamp
	// that happand on _framequeue.init thats why use geter func of sizeofbuffer
	_window.CreateMainWindow(hInstance);
	_workerW.SpawnWorkerW();
	_workerW.FindWorkerW();
	_window.AttachHwndToWorkerW(_workerW.GetWorkerW());
	_dxdevice.CreateDeviceAndDeviceContext();
	_ffmpeg.InitFFmpeg(fileparth, _dxdevice.GetDevice(), 
		_dxdevice.GetDeviceContext(), _framequeue.GetSizeofBuffer());

	_DecodeingLoop_Thread = std::thread([this]()
	{
		_ffmpeg.RunDecoderLoop(
			[this](AVFrame* f, double pts) {_framequeue.push(f, pts); },
			[this]() {return _framepool.GetFrame(); },
			[this](AVFrame* f) {_framepool.ReturnFrame(f); }
		);
		
	});

	_swapchin.CreateSwapChin1(
		_window.GetWindowHeight(),
		_window.GetWindowWidth(),
		_dxdevice.GetDevice());
	_swapchin.CreateRTVForBackBuffer(_dxdevice.GetDevice(), _dxdevice.GetDeviceContext());
	_DComp.CreateDComp(_window.GetHWND(), _swapchin.GetSwapChin(), _dxdevice.GetDevice());
	
	_dxva.InitDXVA(
		_dxdevice.GetDevice(),
		_dxdevice.GetDeviceContext(),
		_swapchin.GetBackBuffer(),
		_ffmpeg.GetCodecContext(),
		_swapchin.GetSwapChinWidth(),
		_swapchin.GetSwapChinHeight()
	);

	_window.ShowMainWindow();

	_window.MessageLoopRun([this]() 
	{
			_render.RenderFrame(_swapchin.GetRTVOfBackBuffer(),
				_swapchin.GetSwapChin(),
				_dxdevice.GetDeviceContext(),
				[this](double &pts) {return _framequeue.pop(pts); },
				[this](AVFrame* f) {_framepool.ReturnFrame(f); },
				[this](AVFrame* f) {_dxva.ProcessFrame(f); }

			);
	});

}