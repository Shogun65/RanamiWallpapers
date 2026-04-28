#include "Render.h"

void Render::StartPlaybackClock(double CurrentPtsSec)
{
	QueryPerformanceFrequency(&_QpcFreq);
	QueryPerformanceCounter(&_QpcStart);
	_FirstPtsSec = CurrentPtsSec;
	_LastPtsSec = CurrentPtsSec;
	_ClockStarted = true;
}

void Render::ResetPlaybackClock(double CurrentPtsSec)
{
	QueryPerformanceCounter(&_QpcStart);
	_FirstPtsSec = CurrentPtsSec;
}

double Render::GetElapsedPlaybackSec() const
{
	LARGE_INTEGER now{};
	QueryPerformanceCounter(&now);

	return double(now.QuadPart - _QpcStart.QuadPart) / double(_QpcFreq.QuadPart);
}

double Render::GetPlaybackTimeSec() const
{
	return _FirstPtsSec + GetElapsedPlaybackSec();
}

double Render::GetFrameDeltaSec(double CurrentPtsSec) const
{
	return CurrentPtsSec - GetPlaybackTimeSec();
}

bool Render::IsLoopRestart(double CurrentPtsSec) const
{
	return CurrentPtsSec + _LoopRestartThresholdSec < _LastPtsSec;
}

bool Render::IsFrameTooLate(double CurrentPtsSec) const
{
	return GetFrameDeltaSec(CurrentPtsSec) < -_LateFrameDropThresholdSec;
}

void Render::WaitUntilPts(double TargetPtsSec) const
{
	double targetElapsedSec = TargetPtsSec - _FirstPtsSec;

	for (;;)
	{
		double remainSec = targetElapsedSec - GetElapsedPlaybackSec();

		if (remainSec <= 0.0)
		{
			return;
		}

		if (remainSec > _Sleep1ThresholdSec)
		{
			Sleep(1);
		}
		else if (remainSec > _YieldThresholdSec)
		{
			Sleep(0);
		}
	}
}

void Render::cleanscreen(
	ID3D11RenderTargetView* RTVOfBackBuffer,
	IDXGISwapChain1* swapchin1,
	ID3D11DeviceContext* devicecontext)
{
	devicecontext->OMSetRenderTargets(
		1,
		&RTVOfBackBuffer,
		nullptr
	);

	float clearcolor[] = {0, 1, 0, 1};

	devicecontext->ClearRenderTargetView(RTVOfBackBuffer, clearcolor);

	HRESULT hr = swapchin1->Present(1, 0);

	if (FAILED(hr))
	{
		printf("Present failed: 0x%08X\n", (unsigned int)hr);
		PostQuitMessage(1);
	}

}
