#include "FFmpeg.h"
#include <algorithm>
/*
*	FrameQueue class in FFmpeg because of i dont want to have more pain
*	soo yea get whit it
* 
*/

void FrameQueue::init(int sizeofbuffer)
{
	_SizeofBuffer = std::clamp(sizeofbuffer, 3, 18);
	_StartThreshold = _SizeofBuffer;
	_Buffer = new AVFrame* [_SizeofBuffer];
	_PtsBuffer = new double[_SizeofBuffer];

	for(int i = 0; i < _SizeofBuffer; i++)
	{
		_Buffer[i] = nullptr;
		_PtsBuffer[i] = 0.0;
	}
	printf("Framequeue init done!\n");
}

FrameQueue::~FrameQueue()
{
	for(int i = 0; i < _SizeofBuffer; i++)
	{
		av_frame_free(&_Buffer[i]);
	}
	delete[] _Buffer;
	delete[] _PtsBuffer;
}

// or you can say Queue size
int FrameQueue::GetSizeofBuffer() const 
{
	printf("Queue size return: %d\n", _SizeofBuffer);
	return _SizeofBuffer;
}

bool FrameQueue::push(AVFrame* Frame, double PtsSec)
{
	std::unique_lock<std::mutex> lock(_Mutex);
	
	_CondFull.wait(lock, [&]() 
	{ 
		return _BufferCount < _SizeofBuffer; // if Queue is full than wait
	});

	//printf("Pushing Frame on Tail: %d\n", _Tail);
	_Buffer[_Tail] = Frame; // put the frame pointer in Queue
	_PtsBuffer[_Tail] = PtsSec;

	_Tail = (_Tail + 1) % _SizeofBuffer; // move the Tail 
	//printf("Next Tail: %d\n", _Tail);

	_BufferCount++; // Add when we get frame
	//printf("BufferCount: %d\n", _BufferCount);

	if(_Buffering && _StartThreshold <= _BufferCount)
	{
		_Buffering = false;
		printf("_Buffering Done!, _BufferCount: %d\n", _BufferCount);
	}

	lock.unlock(); // before the notify_one!

	_CondEmpty.notify_one();

	return true; // dont matter😅
}

AVFrame* FrameQueue::pop(double &OutPtsSec)
{
	std::unique_lock<std::mutex> lock(_Mutex);

	_CondEmpty.wait(lock, [&]()
	{
		return (_BufferCount > 0) && (!_Buffering);
	});
	//printf("Takeing Frame from head: %d\n", _Head);
	AVFrame* frame = _Buffer[_Head];
	OutPtsSec = _PtsBuffer[_Head];

	_Buffer[_Head] = nullptr; // good think to do. not really importand
	_PtsBuffer[_Head] = 0.0;

	_Head = (_Head + 1) % _SizeofBuffer;
	//printf("Next head: %d\n", _Head);
	_BufferCount--;
	//printf("Buffer count: %d\n", _BufferCount);
	lock.unlock();

	_CondFull.notify_one();
	
	return frame;
}