#pragma once

#include <d3d11.h>
#include<wrl/client.h>

using Microsoft::WRL::ComPtr;
#pragma comment(lib, "d3d11.lib")

class DXDevice
{
public:


	void CreateDeviceAndDeviceContext();


	ID3D11Device* GetDevice() const;
	ID3D11DeviceContext* GetDeviceContext() const;

private:
	ComPtr<ID3D11Device> _Device;
	ComPtr<ID3D11DeviceContext> _DeviceContext;
};