from fetch_transcript import fetch_youtube_transcript
from fetch_audio import download_audio
from slice_audio import slice_mp3_from_json
import sys
import argparse

import re

def extract_youtube_id(url):
    youtube_url_regex = r"^(https?://)?(www\.)?youtube\.com/watch\?v=([A-Za-z0-9_-]{11})$"

    match = re.match(youtube_url_regex, url)

    if match:
        return match.group(3)
    return None

def main():
    parser = argparse.ArgumentParser(description="Collect a YouTube URL.")
    parser.add_argument('url', type=str, help='The YouTube URL to process')
    args = parser.parse_args()
    
    youtube_id = extract_youtube_id(args.url)
    print(youtube_id)
    # mp3_file_name = download_audio(args.url)
    # print(mp3_file_name)
    
    transcript_file_name = fetch_youtube_transcript(youtube_id, "my_title")
    

    # # receive a python arg that will be an URL youtbe link, parse it
    # my_url = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"


    # mp3_file_name = download_audio(my_url, output_path=json_file_name)
    # print(mp3_file_name)


    # # logic to fetrch youtube id only!
    # video_id = logic_to_break_url_and_fetch_youtube_id(my_url)


    # json_file_name = fetch_youtube_transcript(video_id=video_id, title="my_title")
    # print(json_file_name)

    # slice_mp3_from_json(mp3_file_name, json_path=json_file_name)

    

#

if __name__ == '__main__':
    main()