#include "SwapChain.h"
#include <iostream>

IDXGISwapChain1* SwapChin::GetSwapChin() const 
{
	return _SwapChin.Get();
}

UINT SwapChin::GetSwapChinWidth() const
{
	return _SwapChinWidth;
}

UINT SwapChin::GetSwapChinHeight() const
{
	return _SwapChinHeight;
}

ID3D11RenderTargetView* SwapChin::GetRTVOfBackBuffer() const
{
	return _RTVForBackBuffer.Get();
}

ID3D11Texture2D* SwapChin::GetBackBuffer() const
{
	return _BackBuffer.Get();
}

void SwapChin::CreateSwapChin1(LONG Height, LONG Width, ID3D11Device* device)
{
	HRESULT hr;
	hr = CreateDXGIFactory2(0, __uuidof(IDXGIFactory2), (void**)_Factory.GetAddressOf());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"Error on CreateDXGIFactory2", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	DXGI_SWAP_CHAIN_DESC1 scd = { };

	scd.AlphaMode = DXGI_ALPHA_MODE_IGNORE;
	scd.BufferCount = 3;
	scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
	scd.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
	scd.SampleDesc.Count = 1;
	scd.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
	scd.Height = Height;
	scd.Width = Width;
	scd.Flags = DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT;

	hr = _Factory->CreateSwapChainForComposition(
		device,
		&scd,
		nullptr,
		_SwapChin.GetAddressOf()
	);

	if(FAILED(hr))
	{
		MessageBox(nullptr, L"Error on CreateSwapChainForComposition", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}
	else
	{
		printf("CreateSwapChainForComposition done!\n");
	}

}

void SwapChin::CreateRTVForBackBuffer(ID3D11Device* device, ID3D11DeviceContext* devicecontext)
{
	HRESULT hr;

	hr = _SwapChin->GetBuffer(0, __uuidof(ID3D11Texture2D), (void**)_BackBuffer.GetAddressOf());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"Error on GetBuffer", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	device->CreateRenderTargetView(
		_BackBuffer.Get(),
		nullptr,
		_RTVForBackBuffer.GetAddressOf()
	);

	D3D11_TEXTURE2D_DESC desc = { };

	_BackBuffer->GetDesc(&desc);

	_SwapChinWidth = desc.Width;
	_SwapChinHeight = desc.Height;

	D3D11_VIEWPORT vp = {0};

	vp.Height = (FLOAT)_SwapChinHeight;
	vp.Width = (FLOAT)_SwapChinWidth;
	vp.MaxDepth = 1;

	devicecontext->RSSetViewports(
		1,
		&vp
	);
	printf("Backbuffer done!\n");
}
