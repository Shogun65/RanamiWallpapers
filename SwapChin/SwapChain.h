#pragma once
#include <dxgi1_3.h>
#include <d3d11.h>
#include <wrl/client.h>
#pragma comment(lib,"dxgi.lib")

using Microsoft::WRL::ComPtr;

class SwapChin
{
public:

	void CreateSwapChin1(LONG Height, LONG Width, ID3D11Device* device);
	void CreateRTVForBackBuffer(ID3D11Device* device, ID3D11DeviceContext* devicecontext);

	IDXGISwapChain1* GetSwapChin() const;
	UINT GetSwapChinWidth() const;
	UINT GetSwapChinHeight() const;
	ID3D11RenderTargetView* GetRTVOfBackBuffer() const;
	ID3D11Texture2D* GetBackBuffer() const;

private:
	ComPtr<IDXGISwapChain1> _SwapChin = nullptr;
	ComPtr<IDXGIFactory2> _Factory = nullptr;
	ComPtr<ID3D11RenderTargetView> _RTVForBackBuffer = nullptr;
	ComPtr<ID3D11Texture2D> _BackBuffer = nullptr;
	UINT _SwapChinWidth = 0;
	UINT _SwapChinHeight = 0;
};