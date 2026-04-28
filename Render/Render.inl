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

	if (!_ClockStarted)
	{
		StartPlaybackClock(_ptsSec);
	}

	if (IsLoopRestart(_ptsSec))
	{
		ResetPlaybackClock(_ptsSec);
	}

	if (IsFrameTooLate(_ptsSec))
	{
		FrameReturn(_POPFrame);
		_POPFrame = nullptr;
		return;
	}

	double frameDeltaSec = GetFrameDeltaSec(_ptsSec);
	if (frameDeltaSec > 0.0)
	{
		WaitUntilPts(_ptsSec);
	}

	_LastPtsSec = _ptsSec;

	if (!ProcessFrame(_POPFrame))
	{
		FrameReturn(_POPFrame);
		_POPFrame = nullptr;
		PostQuitMessage(1);
		return;
	}

	HRESULT hr = swapchin1->Present(0, 0);
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
