from youtube_transcript_api import YouTubeTranscriptApi
from parse_transcript import parse_date


def fetch_youtube_transcript(video_id, title):
    try:
        transcript = YouTubeTranscriptApi.get_transcript(
            video_id,
            languages=[
                "de",
            ],
        )

        safe_title = "".join([c if c.isalnum() else "_" for c in title])
        file_name = f"{safe_title}.txt"

        with open(file_name, "w", encoding="utf-8") as f:
            for entry in transcript:
                f.write(f"{entry['start']:.2f}s: {entry['text']}\n")

        parse_date(file_name)
        return file_name + ".json"

    except Exception as e:
        from IPython import embed; embed()
        print(f"Error fetching transcript: {e}")


if __name__ == "__main__":
    video_id = "kUHXUDrJwtA"
    title = "german_test_video"
    fetch_youtube_transcript(video_id, title)
