template<typename Pushframe, typename GetFrame, typename ReturnFrame>
void FFmpeg::RunDecoderLoop(Pushframe pushframe, GetFrame getframe, ReturnFrame returnframe)
{
	printf("Decoder thread Runing!\n");
	AVPacket* Packet = av_packet_alloc();

	while (_DecodedThreadruning)
	{
		int ret = av_read_frame(_FormatContext, Packet);
		if (ret == AVERROR_EOF)
		{
			printf("Looping video\n");

			av_packet_unref(Packet);

			ret = av_seek_frame(_FormatContext, _VideoStreamIndex, 0, AVSEEK_FLAG_BACKWARD);
			if (ret < 0)
			{
				printf("Seek failed on loop!\n");
				break;
			}

			// IMPORTANT: flush decoder buffers
			//avcodec_flush_buffers(_CodecContext);

			continue; // keep looping
		}
		else if(ret != 0)
		{
			printf("Erron on decoder!\n");
			break;
		}
		if (Packet->stream_index == _VideoStreamIndex)
		{
			int sendRet = avcodec_send_packet(_CodecContext, Packet);
			if (sendRet < 0)
			{
				printf("Decoder send packet error!\n");
				av_packet_unref(Packet);
				continue;
			}
			
			while (true)
			{
				//printf("Tring to get a AVFrame\n");
				AVFrame* frame = getframe();
				//printf("Got a frame\n");

				int ret = avcodec_receive_frame(_CodecContext, frame);

				if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF)
				{
					//printf("EAGAIN\n");
					returnframe(frame);
					break;
				}
				else if (ret < 0)
				{
					printf("Decoder error!\n");
					returnframe(frame);
					break;
				}
				else if (ret == 0)
				{
					//printf("Pushing Frame to Queue\n");

					int64_t ts = frame->best_effort_timestamp;
					double ptsSec = 0.0;
					if (ts != AV_NOPTS_VALUE) {
						ptsSec = ts * av_q2d(_VideoTimeBase);
					}
					pushframe(frame, ptsSec);
				}
			}

		}
		av_packet_unref(Packet);
	}
	av_packet_free(&Packet);
	printf("stop decoder\n");
}
