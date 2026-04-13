#pragma once
#include <dcomp.h>
#include <dxgi1_2.h>
#include <d3d11.h>
#include <wrl/client.h>
#pragma comment(lib,"dcomp.lib")

using Microsoft::WRL::ComPtr;

class DComp
{
public:

	void CreateDComp(HWND mainwindow, IDXGISwapChain1 *swapchin1, ID3D11Device* device);

private:
	ComPtr<IDXGIDevice> _DXGIDevice = nullptr;
	ComPtr<IDCompositionDevice> _DCompDevice = nullptr;
	ComPtr<IDCompositionVisual> _DCompVisual = nullptr;
	ComPtr<IDCompositionTarget> _DCompTarget = nullptr;
};