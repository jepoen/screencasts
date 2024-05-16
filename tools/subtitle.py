'''
Adapted from https://www.digitalocean.com/community/tutorials/how-to-generate-and-add-subtitles-to-videos-using-python-openai-whisper-and-ffmpeg

srt ditn't work, so changed output to vtt
'''
import argparse
import math
import os
import time
import ffmpeg
from faster_whisper import WhisperModel

def extract_audio(input_video, extracted_audio):
    stream = ffmpeg.input(input_video)
    stream = ffmpeg.output(stream, extracted_audio)
    ffmpeg.run(stream, overwrite_output=True)
    return extracted_audio

def transcribe(audio, model="small"):
    model = WhisperModel(model)
    segments, info = model.transcribe(audio)
    language = info[0]
    print("Transcription language", info[0])
    segments = list(segments)
    for segment in segments[:5]:
        # print(segment)
        print("[%.2fs -> %.2fs] %s" %
              (segment.start, segment.end, segment.text))
    print('...')
    for segment in segments[-5:]:
        # print(segment)
        print("[%.2fs -> %.2fs] %s" %
              (segment.start, segment.end, segment.text))
    return language, segments

def format_time(seconds):
    hours = math.floor(seconds / 3600)
    seconds %= 3600
    minutes = math.floor(seconds / 60)
    seconds %= 60
    milliseconds = round((seconds - math.floor(seconds)) * 1000)
    seconds = math.floor(seconds)
    formatted_time = \
      f"{hours:02d}:{minutes:02d}:{seconds:02d}.{milliseconds:03d}"
    return formatted_time

def generate_subtitle_file(input_video_name, language, segments):
    subtitle_file = f"{input_video_name}-sub.{language}.vtt"
    text = "WEBVTT\n\n"
    for index, segment in enumerate(segments):
        segment_start = format_time(segment.start)
        segment_end = format_time(segment.end)
        #text += f"{str(index+1)}\n"
        text += f"{segment_start} --> {segment_end}\n"
        text += f"{segment.text}\n"
        text += "\n"
    f = open(subtitle_file, "w")
    f.write(text)
    f.close()

    return subtitle_file

def run():
  parser = argparse.ArgumentParser(
    prog='subtitle',
    description='Generate video suptitles using whisper',
  )
  parser.add_argument('-m', '--model', help='whisper model name',
    default='small')
  parser.add_argument('-pa', '--preserve-audio', help='preserve audio file',
    action='store_true')
  parser.add_argument('filename', help='video file name')
  args = parser.parse_args()
  #print(args.filename, args.model)
  input_video = args.filename
  input_video_name, _ext = os.path.splitext(input_video)
  #model = 'large-v3'
  #model = 'medium'
  #model = 'distil-large-v3'
  model = args.model
  extracted_audio = f"{input_video_name}-audio.wav"
  extract_audio(input_video, extracted_audio)
  language, segments = transcribe(extracted_audio, model)
  if not args.preserve_audio:
    print('Delete audio file {}'.format(extracted_audio))
    os.remove(extracted_audio)
  print('Language: {}, model: {}'.format(language, model))
  subtitle_file = generate_subtitle_file(
    input_video_name,
    language=language,
    segments=segments
  )
  print('Result: {}'.format(subtitle_file))

if __name__ == '__main__':
  run()

