#include "FFmpeg.h"
#include <iostream>

// it take the size of queue that is clamp to 3 to 18 max and init X amount of
// AVFrame is memory simple as that
void FramePool::init(int sizeofqueue)
{
	_FramePool.reserve(sizeofqueue);

	for(int i = 0; i < sizeofqueue; i++)
	{
		printf("alloc AVFRAME!!\n");
		AVFrame* Frame = av_frame_alloc();
		_FramePool.push_back(Frame);
	}
}

void FramePool::Shutdown()
{
	std::lock_guard<std::mutex> lock(_Mutex);
	_Shutdown = true;
	_Cond.notify_all();
}

AVFrame* FramePool::GetFrame()
{
	std::unique_lock<std::mutex> lock(_Mutex);

	//printf("GetFrame before wait\n");
	//printf("FramePool size before getframe: %zd\n", _FramePool.size());
	_Cond.wait(lock, [&]()// wait need false to wait okay and true mean dont wait
	{
		return _Shutdown || !_FramePool.empty();
	});

	if (_Shutdown)
	{
		return nullptr;
	}

	AVFrame* Frame = _FramePool.back();
	_FramePool.pop_back();

	//printf("FramePool size after getframe: %zd\n", _FramePool.size());

	return Frame;
}

void FramePool::ReturnFrame(AVFrame* frame)
{
	if (!frame)
	{
		return;
	}

	std::unique_lock<std::mutex> lock(_Mutex);

	if (_Shutdown)
	{
		return;
	}

	av_frame_unref(frame);

	_FramePool.push_back(frame);
	//printf("ReturnFrame called!\n");
	lock.unlock();

	_Cond.notify_one();
}
