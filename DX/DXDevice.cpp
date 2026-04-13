#include "DXDevice.h"
#include <iostream>

ID3D11Device* DXDevice::GetDevice() const
{
	return _Device.Get();
}

ID3D11DeviceContext* DXDevice::GetDeviceContext() const 
{
	return _DeviceContext.Get();
}

void DXDevice::CreateDeviceAndDeviceContext()
{
	HRESULT hr;

	hr = D3D11CreateDevice(
		nullptr,
		D3D_DRIVER_TYPE_HARDWARE,
		0,
		0,
		nullptr,
		0,
		D3D11_SDK_VERSION,
		_Device.GetAddressOf(),
		nullptr,
		_DeviceContext.GetAddressOf()
	);

	if(FAILED(hr))
	{
		MessageBox(nullptr, L"DXDevice Create error", L"DXDeive Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}
	else
	{
		printf("Device and DeviceContext Created!\n");
	}

}