#pragma once

#include <d3d11.h>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <vector>
extern "C" {
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libswscale/swscale.h>
#include <libswresample/swresample.h>
#include <libavutil/hwcontext_d3d11va.h>
#include <libavutil/pixdesc.h>
#include <libavutil/frame.h>
}


class FramePool
{
public:

	void init(int sizeofqueue);
	AVFrame* GetFrame();
	void ReturnFrame(AVFrame* frame);

private:

	std::mutex _Mutex;
	std::vector<AVFrame*> _FramePool;
	std::condition_variable _Cond;
};

class FrameQueue
{
public:

	~FrameQueue();

	void init(int sizeofbuffer = 3);
	int GetSizeofBuffer() const;
	AVFrame* pop(double& OutPtsSec);
	bool push(AVFrame* Frame, double PtsSec);

private:

	AVFrame** _Buffer = nullptr; // AVFrame** that we useing to store the pointer of AVFrame*

	int _SizeofBuffer = 3; // A safe defualt
	double* _PtsBuffer;
	int _Head = 0; // read
	int _Tail = 0; // write
	int _BufferCount = 0;
	std::mutex _Mutex;
	std::condition_variable _CondFull;
	std::condition_variable _CondEmpty;
	bool _Buffering = true; // renderer waits at start
	int _StartThreshold = 0; // how many frames before render starts

};

class FFmpeg
{
public:

	void InitFFmpeg(const char* fileparth,
		ID3D11Device* Device, ID3D11DeviceContext* DeviceContext, int sizeofbuffer);

	static AVPixelFormat get_pix_format(
		AVCodecContext* CodecCtx,
		const AVPixelFormat* pix_fmt);

	AVCodecContext* GetCodecContext() const;

	void ShutDownDecoder();

	template<typename Pushframe, typename GetFrame, typename ReturnFrame>
	void RunDecoderLoop(Pushframe pushframe, GetFrame getframe, ReturnFrame returnframe);

private:
	AVBufferRef* _HWDevice = nullptr;
	AVBufferRef* _HWFrame = nullptr;
	AVFormatContext* _FormatContext = nullptr;
	int _VideoStreamIndex = -1; // dont change this
	const AVCodec* _Codec = nullptr;
	AVCodecParameters* _CodecParameter = nullptr;
	AVCodecContext* _CodecContext = nullptr;
	AVRational _VideoTimeBase{0, 1};
	/*
	*	This is going to run on deffrent thread. when it run when it stop?
	*	This Loop start when FFmpeginit func done!.	
	*	When it stop well when the Whole EXE (dll whatever) exit..
	*
	*/
	std::atomic<bool> _DecodedThreadruning = true; // this is improtand
};
#include "DecoderLoop.inl"
