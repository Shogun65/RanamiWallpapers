#include "FFmpeg.h"
#include <cstdlib>

/*
*	This
* 
* 
*/
void FFmpeg::InitFFmpeg(const char* fileparth, 
	ID3D11Device* Device, ID3D11DeviceContext* DeviceContext, int sizeofbuffer)
{
	_HWDevice = av_hwdevice_ctx_alloc(AV_HWDEVICE_TYPE_D3D11VA);

	AVHWDeviceContext* HWDeviceCtx = (AVHWDeviceContext*)_HWDevice->data;

	AVD3D11VADeviceContext* d3d11DeviceContext = (AVD3D11VADeviceContext*)HWDeviceCtx->hwctx;

	d3d11DeviceContext->device = Device;
	d3d11DeviceContext->device_context = DeviceContext;


	if(av_hwdevice_ctx_init(_HWDevice) < 0)
	{
		MessageBoxW(nullptr, L"Error on av_hwdevice_ctx_init", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}
	else
	{
		printf("hwDevice init Done!\n");
	}

	if(avformat_open_input(&_FormatContext, fileparth, nullptr, nullptr) < 0)
	{
		MessageBoxW(nullptr, L"Error on avformat_open_input", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	if(avformat_find_stream_info(_FormatContext, nullptr) < 0)
	{
		MessageBoxW(nullptr, L"Error on avformat_find_stream_info", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	for(unsigned int i = 0; i < _FormatContext->nb_streams; i++)
	{
		if (_FormatContext->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO)
		{
			_VideoStreamIndex = i;
			break;
		}
	}
	
	if(_VideoStreamIndex == -1)
	{
		MessageBox(nullptr, L"_VideoStreamIndex still -1", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	_VideoTimeBase = _FormatContext->streams[_VideoStreamIndex]->time_base;

	_CodecParameter = _FormatContext->streams[_VideoStreamIndex]->codecpar;

	_Codec = avcodec_find_decoder(_CodecParameter->codec_id);

	_CodecContext = avcodec_alloc_context3(_Codec);

	avcodec_parameters_to_context(_CodecContext, _CodecParameter);

	_CodecContext->hw_device_ctx = _HWDevice;
	_CodecContext->get_format = get_pix_format;
	_CodecContext->sw_pix_fmt = AV_PIX_FMT_NV12;

	_HWFrame = av_hwframe_ctx_alloc(_HWDevice);

	if (!_HWFrame)
	{
		MessageBox(nullptr, L"Error on av_hwframe_ctx_alloc", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	AVHWFramesContext* FrameCtx = (AVHWFramesContext*)_HWFrame->data;

	AVD3D11VAFramesContext* D3D11Ctx = (AVD3D11VAFramesContext*)FrameCtx->hwctx;
	D3D11Ctx->BindFlags = D3D11_BIND_DECODER | D3D11_BIND_SHADER_RESOURCE;

	FrameCtx->width = _CodecContext->width;
	FrameCtx->height = _CodecContext->height;
	FrameCtx->format = AV_PIX_FMT_D3D11;
	FrameCtx->sw_format = AV_PIX_FMT_NV12;
	// sizeofbuffer is clamp soo it can be min 3 and max 18. that means max pool can be
	// 24 and the less pool can be 9 because of that 6 we adding..
	// soo no need to clamp here.. why adding 6 because pool needed to be bigger than
	// frame queue! 6 is safe number to add more pool than queue
	FrameCtx->initial_pool_size = (sizeofbuffer + 6);

	if (av_hwframe_ctx_init(_HWFrame) < 0)
	{
		MessageBox(nullptr, L"Error on av_hwframe_ctx_init", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}

	_CodecContext->hw_frames_ctx = av_buffer_ref(_HWFrame);

	if (avcodec_open2(_CodecContext, _Codec, nullptr) < 0)
	{
		MessageBox(nullptr, L"Error on avcodec_open2", L"Error", MB_ICONERROR);
		std::exit(EXIT_FAILURE);
	}
	else
	{
		printf("FFmpeginit done!\n");
	}
}


/*
*
*	You know what?.. i still kinda have no idea how this work but it work.
* 
*/
AVPixelFormat FFmpeg::get_pix_format
(
	AVCodecContext* CodecCtx,
	const AVPixelFormat* pix_fmt
)
{
	for(const enum AVPixelFormat *p = pix_fmt; *p != AV_PIX_FMT_NONE; p++)
	{
		if(*p == AV_PIX_FMT_D3D11)
		{
			printf("AV_PIX_FMT_D3D11\n");
			return *p;
		}
	}
	printf("AV_PIX_FMT_YUV420P\n");
	return AV_PIX_FMT_YUV420P;
}

AVCodecContext* FFmpeg::GetCodecContext() const 
{
	return _CodecContext;
}

void FFmpeg::ShutDownDecoder()
{
	this->_DecodedThreadruning = false; //idk why i use this but it look cool hehe
}
