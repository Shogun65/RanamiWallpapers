#pragma once
#include <d3d11.h>
#include<wrl/client.h>

extern "C"
{
#include <libavcodec/avcodec.h>
}

using Microsoft::WRL::ComPtr;

class DXVA
{
public:

	/**
	*	Call this Func After DX and Swapchin are done.
	*	
	*	This func take care of most of createing stuff and init. so you 
	*	dont need to worry much about init and createing stuff by your self.
	*	NOTE:about those Width and Hight are swapchin W and H. so dont get confused!
	*/
	void InitDXVA(
		ID3D11Device* Device,
		ID3D11DeviceContext* DeviceContext,
		ID3D11Texture2D* BackBuffer,
		AVCodecContext* CodecContext,
		UINT DWidth, UINT DHeight
		);

	/**
	*	This Func Process the AVFrame that we pop in render than
	*	render that frame on backbuffer than we can persent!
	*/
	void ProcessFrame(AVFrame* POPFrame);

private:
	ComPtr<ID3D11VideoDevice> _VideoDevice = nullptr;
	ComPtr<ID3D11VideoContext> _VideoContext = nullptr;
	ComPtr<ID3D11VideoProcessor> _VideoProcessor = nullptr;
	ComPtr<ID3D11VideoProcessorEnumerator> _VideoProcessorEnum = nullptr;
	ComPtr<ID3D11VideoProcessorInputView> _VideoInputView = nullptr;
	ComPtr<ID3D11VideoProcessorOutputView> _VideoOutputView = nullptr;

	void CreateOutputView(ID3D11Texture2D* BackBuffer);
	void InitVideoDeviceAndContext(ID3D11Device* Device,
		ID3D11DeviceContext* DeviceContext);
	void ProcessVideoColor(AVCodecContext* CodecContext);
	
	//Those Width and Height are Swapchin W and H.
	void ProcessVideoWidthAndHeight(
		AVCodecContext* CodecContext, 
		UINT DWidth, UINT DHeight);

	/**
	*	This Func get call by ProcessFrame where to call? 
	*	On where he need inputview of texture2d there you just call this func and
	*	he reture the ID3D11VideoProcessorInputView*.
	*	NOTE: GetInputView use AVFrame data [0] and [1] to make inputview
	*	means zero copy
	*/
	ID3D11VideoProcessorInputView* GetInputView(AVFrame* POPFrame);
	
};
