#include "Render.h"

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
