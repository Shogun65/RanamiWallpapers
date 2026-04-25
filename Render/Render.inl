template<typename FramePOPFunc, typename FrameReturnFunc, typename ProcessFrameFunc>
void Render::RenderFrame(
	ID3D11RenderTargetView* RTVOfBackBuffer,
	IDXGISwapChain1* swapchin1,
	ID3D11DeviceContext* devicecontext,
	FramePOPFunc FramePOP,
	FrameReturnFunc FrameReturn,
	ProcessFrameFunc ProcessFrame)
{

	float green_color[] = {0, 1, 0, 1};

	devicecontext->OMSetRenderTargets(
		1,
		&RTVOfBackBuffer,
		nullptr
		);

	devicecontext->ClearRenderTargetView(
		RTVOfBackBuffer,
		green_color
	);

	if (_POPFrame == nullptr) 
	{
		_ptsSec = 0.0;
		_POPFrame = FramePOP(_ptsSec);
		if (_POPFrame == nullptr)
		{
			return;
		}
	}

	if (!_ClockStarted) {
		QueryPerformanceFrequency(&_QpcFreq);
		QueryPerformanceCounter(&_QpcStart);
		_FirstPtsSec = _ptsSec;
		_LastPtsSec = _ptsSec;
		_ClockStarted = true;
	}

	// loop restart detect: pts jumps backwards
	if (_ptsSec + 0.5 < _LastPtsSec) {
		QueryPerformanceCounter(&_QpcStart);
		_FirstPtsSec = _ptsSec;
	}
	_LastPtsSec = _ptsSec;

	LARGE_INTEGER now{};
	QueryPerformanceCounter(&now);

	double elapsedSec = double(now.QuadPart - _QpcStart.QuadPart) / double(_QpcFreq.QuadPart);
	double targetSec = _ptsSec - _FirstPtsSec;
	double waitSec = targetSec - elapsedSec;

	if (waitSec > 0.0) {
		DWORD ms = (DWORD)(waitSec * 1000.0);
		if (ms > 0) Sleep(ms);
	}

	if (!ProcessFrame(_POPFrame))
	{
		FrameReturn(_POPFrame);
		_POPFrame = nullptr;
		PostQuitMessage(1);
		return;
	}

	HRESULT hr = swapchin1->Present(1, 0);
	if (FAILED(hr))
	{
		printf("Present failed: 0x%08X\n", (unsigned int)hr);
		FrameReturn(_POPFrame);
		_POPFrame = nullptr;
		PostQuitMessage(1);
		return;
	}

	FrameReturn(_POPFrame);
	_POPFrame = nullptr;
}
