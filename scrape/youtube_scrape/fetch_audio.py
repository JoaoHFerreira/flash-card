import os
import yt_dlp
from pydub import AudioSegment


def download_audio(youtube_url, output_path="."):
    temp_file = None
    ydl_opts = {
        "format": "bestaudio/best",
        "outtmpl": f"{output_path}/%(title)s.%(ext)s",
    }

    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(youtube_url, download=True)
            temp_file = ydl.prepare_filename(info)

        if temp_file and temp_file.endswith(".webm"):
            audio = AudioSegment.from_file(temp_file, format="webm")
            mp3_filename = temp_file.replace(".webm", ".mp3")
            audio.export(mp3_filename, format="mp3")
            print(f"Audio converted to MP3: {mp3_filename}")

            os.remove(temp_file)
            print(f"Temporary file removed: {temp_file}")
            return mp3_filename

        if temp_file and temp_file.endswith(".m4a"):
            audio = AudioSegment.from_file(temp_file, format="m4a")
            mp3_filename = temp_file.replace(".m4a", ".mp3")
            audio.export(mp3_filename, format="mp3")
            print(f"Audio converted to MP3: {mp3_filename}")

            os.remove(temp_file)
            print(f"Temporary file removed: {temp_file}")
            return mp3_filename
        
        else:
            from IPython import embed
            embed()
            print("Downloaded file is not in .webm format or no file was downloaded.")

    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    youtube_url = "https://www.youtube.com/watch?v=kUHXUDrJwtA"
    download_audio(youtube_url)
