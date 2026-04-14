#pragma once

#include "Window/Window.h"
#include "Window/WorkerW.h"
#include "DX/DXDevice.h"
#include "SwapChin/SwapChain.h"
#include "DComp/DComp.h"
#include "Render/Render.h"
#include "FFmpeg/FFmpeg.h"
#include "DXVA/DXVA.h"

class Engine
{
public:

//This is all funs just for Early stage testing
	void CreateNormalWindowAndRun(HINSTANCE Hinstance);
	void CreateWindowOnWorkerWAndRun(HINSTANCE hInstance);
	void SeeWindowTree();
	void testDx();
	void testDXandSwapchin(HINSTANCE hInstance);
	void testDXandswapanddcomp(HINSTANCE hInstance);
	void MakeWindowRunwhitWorkerWandRunDXandswapchin(HINSTANCE hInstance);
	void MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(HINSTANCE hInstance, 
		const char* filepart, int sizeofbuffer);
//  ^^^^^^^^^^^^^^^^^^^

	~Engine();


private:
	Window _window;
	WorkerW _workerW;
	DXDevice _dxdevice;
	SwapChin _swapchin;
	DComp _DComp;
	Render _render;
	FFmpeg _ffmpeg;
	FrameQueue _framequeue;
	FramePool _framepool;
	DXVA _dxva;

	//this is for decoder loop
	std::thread _DecodeingLoop_Thread;
};