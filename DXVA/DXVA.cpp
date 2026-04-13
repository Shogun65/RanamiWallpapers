#include "DXVA.h"
#include <cstdlib>

void DXVA::InitVideoDeviceAndContext(
	ID3D11Device* Device,
	ID3D11DeviceContext* DeviceContext)
{
	HRESULT hr;
	hr = Device->QueryInterface(
		__uuidof(ID3D11VideoDevice),
		(void**)_VideoDevice.GetAddressOf()
	);

	hr = DeviceContext->QueryInterface(
		__uuidof(ID3D11VideoContext),
		(void**)_VideoContext.GetAddressOf()
	);

	if(FAILED(hr))
	{
		MessageBox(nullptr, L"Error on InitVideoDeviceAndContext Func", L"Error on DXVA",
			MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

}

void DXVA::ProcessVideoWidthAndHeight(AVCodecContext* CodecContext, UINT DWidth, UINT DHeight)
{
	RECT srcRECT = { 0, 0, CodecContext->width, CodecContext->height };
	RECT dstRECT = { 0, 0, (LONG)DWidth, (LONG)DHeight };

	_VideoContext->VideoProcessorSetStreamDestRect(
		_VideoProcessor.Get(),
		1,
		TRUE,
		&dstRECT
	);

	_VideoContext->VideoProcessorSetStreamSourceRect(
		_VideoProcessor.Get(),
		0,
		TRUE,
		&srcRECT
	);
}

void DXVA::CreateOutputView(ID3D11Texture2D* BackBuffer)
{
	HRESULT hr;
	D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC ovdesc = { };
	ovdesc.ViewDimension = D3D11_VPOV_DIMENSION_TEXTURE2D;

	hr = _VideoDevice->CreateVideoProcessorOutputView(
		BackBuffer,
		_VideoProcessorEnum.Get(),
		&ovdesc,
		_VideoOutputView.GetAddressOf()
	);

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"Error on CreateOutputView Func", L"Error on DXVA",
			MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

}

void DXVA::ProcessVideoColor(AVCodecContext* CodecContext)
{
	D3D11_VIDEO_PROCESSOR_COLOR_SPACE csdesc = { };

	csdesc.Usage = 0;
	csdesc.RGB_Range = (CodecContext->color_range == AVCOL_RANGE_JPEG) ? 1 : 0;

	switch (CodecContext->colorspace)
	{
	case AVCOL_SPC_BT709:
		csdesc.YCbCr_Matrix = 1;
		break;

	case AVCOL_SPC_BT470BG:
	case AVCOL_SPC_SMPTE170M:
		csdesc.YCbCr_Matrix = 0;
		break;

	default:
		csdesc.YCbCr_Matrix = 1;
		break;
	}

	csdesc.Nominal_Range = 0;
	csdesc.YCbCr_xvYCC = 0;

	_VideoContext->VideoProcessorSetStreamColorSpace(_VideoProcessor.Get(), 0, &csdesc);
	_VideoContext->VideoProcessorSetOutputColorSpace(_VideoProcessor.Get(), &csdesc);
}

ID3D11VideoProcessorInputView* DXVA::GetInputView(AVFrame* POPFrame)
{
	HRESULT hr;
	ID3D11Texture2D* NV12Frame = (ID3D11Texture2D*)POPFrame->data[0];

	int subresorce = (int)(intptr_t)POPFrame->data[1];

	D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC ivdesc = { };

	ivdesc.Texture2D.ArraySlice = subresorce; // verry improtand
	ivdesc.ViewDimension = D3D11_VPIV_DIMENSION_TEXTURE2D;

	hr = _VideoDevice->CreateVideoProcessorInputView(
		NV12Frame,
		_VideoProcessorEnum.Get(),
		&ivdesc,
		_VideoInputView.ReleaseAndGetAddressOf()
		);

	if (FAILED(hr))
	{
		MessageBox(nullptr, L"Error on GetInputView Func", L"Error on DXVA",
			MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	return _VideoInputView.Get();
}

void DXVA::ProcessFrame(AVFrame* POPFrame)
{
	D3D11_VIDEO_PROCESSOR_STREAM vps = { };
	vps.Enable = TRUE;
	vps.pInputSurface = GetInputView(POPFrame);
	
	_VideoContext->VideoProcessorBlt(
		_VideoProcessor.Get(),
		_VideoOutputView.Get(),
		0,
		1,
		&vps
	);

}

void DXVA::InitDXVA(
	ID3D11Device* Device,
	ID3D11DeviceContext* DeviceContext,
	ID3D11Texture2D* BackBuffer,
	AVCodecContext* CodecContext,
	UINT DWidth, UINT DHeight)
{
	HRESULT hr;
	InitVideoDeviceAndContext(Device, DeviceContext);
	
	D3D11_VIDEO_PROCESSOR_CONTENT_DESC vpcdesc = { };

	vpcdesc.Usage = D3D11_VIDEO_USAGE_PLAYBACK_NORMAL;
	vpcdesc.InputWidth = CodecContext->width;
	vpcdesc.InputHeight = CodecContext->height;
	vpcdesc.OutputWidth = DWidth;
	vpcdesc.OutputHeight = DHeight;
	vpcdesc.InputFrameFormat = D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE;

	hr = _VideoDevice->CreateVideoProcessorEnumerator(
		&vpcdesc,
		_VideoProcessorEnum.GetAddressOf()
	);

	hr = _VideoDevice->CreateVideoProcessor(
		_VideoProcessorEnum.Get(), 
		0, 
		_VideoProcessor.GetAddressOf()
	);


	if (FAILED(hr))
	{
		MessageBox(nullptr, L"Error on InitDXVA Func", L"Error on DXVA",
			MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	ProcessVideoColor(CodecContext);
	ProcessVideoWidthAndHeight(CodecContext, DWidth, DHeight);
	CreateOutputView(BackBuffer);
}

