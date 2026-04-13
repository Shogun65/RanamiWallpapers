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

	swapchin1->Present(1, 0);

}