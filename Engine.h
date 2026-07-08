#pragma once

#include "Window/Window.h"
#include "Window/WorkerW.h"
#include "DX/DXDevice.h"
#include "SwapChin/SwapChain.h"
#include "DComp/DComp.h"
#include "Render/Render.h"
#include "FFmpeg/FFmpeg.h"
#include "DXVA/DXVA.h"
#include "Parse/Parse.h"
#include "PostMessageW/PostMessageW.h"
#include <iostream>

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
	void MakeWindowRunwhitWorkerWandRunDXandswapchinWhitFFmpeg(HINSTANCE hInstance);
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
	Parse _parse;
	MyPostMessage _mypostmessage;

	//this is for decoder loop
	std::thread _DecodeingLoop_Thread;

	// some windows message here check docs for mroe info (hopefully i update tham)

	uint32_t WM_ENGINE_SENT_HWND = WM_APP + 5;
};