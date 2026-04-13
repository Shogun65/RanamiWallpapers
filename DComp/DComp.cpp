#include "DComp.h"
#include <iostream>

void DComp::CreateDComp(HWND mainwindow, IDXGISwapChain1 *swapchin1, ID3D11Device* device)
{
	HRESULT hr;

	hr = device->QueryInterface(
		__uuidof(IDXGIDevice),
		(void**)_DXGIDevice.GetAddressOf());

	if(FAILED(hr))
	{
		MessageBox(nullptr, L"error on _DCompDevice->QueryInterface", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}


	hr = DCompositionCreateDevice(_DXGIDevice.Get(),
		__uuidof(IDCompositionDevice), 
		(void**)_DCompDevice.GetAddressOf());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on DCompositionCreateDevice", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	hr = _DCompDevice->CreateTargetForHwnd(
		mainwindow,
		TRUE,
		_DCompTarget.GetAddressOf());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on CreateTargetForHwnd", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	hr = _DCompDevice->CreateVisual(_DCompVisual.GetAddressOf());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on CreateVisual", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	hr = _DCompVisual->SetContent(swapchin1);

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on SetContent", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	hr = _DCompTarget->SetRoot(_DCompVisual.Get());

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on SetRoot", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	hr = _DCompDevice->Commit();

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"error on commit", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}
	else
	{
		printf("DComp done!\n");
	}
	// dont laugh okay
}