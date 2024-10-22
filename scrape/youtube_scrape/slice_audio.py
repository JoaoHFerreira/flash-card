import json
import os
from pydub import AudioSegment


def slice_mp3_from_json(input_file, json_path, output_dir="sliced_data"):
  """Slices an MP3 file based on start and end times specified in a JSON file and saves
  the segments with a numbered pattern using pydub library.

  Args:
      input_file (str): Path to the input MP3 file.
      output_dir (str): Path to the output directory.
      json_data (list): List of dictionaries containing 'init_time' and 'final_time' values.
  """

  with open(json_path, "r") as f:
    json_data = json.load(f)

  os.makedirs(output_dir, exist_ok=True)

  sound = AudioSegment.from_mp3(input_file)

  # Iterate over the JSON data and slice the audio
  for i, data in enumerate(json_data):
    start_time = float(data['init_time']) * 1000  # Convert seconds to milliseconds for pydub
    end_time = float(data['final_time']) * 1000 if 'final_time' in data else len(sound)  # Handle missing final time

    # Extract the audio segment
    sliced_audio = sound[int(start_time):int(end_time)]

    # Create the output file name
    output_file = os.path.join(output_dir, f"file_name_part{i+1}.mp3")

    # Export the sliced audio segment as a new MP3 file
    sliced_audio.export(output_file, format="mp3")


if __name__ == "__main__":
  input_file = "Ist die Lehre wirklich ein Ausbildungsweg zweiter Klasse？ ｜ Claudia Plakolm ｜ TEDxSalzburg.mp3"
  output_dir = "sliced_audio"
  


  slice_mp3_from_json(input_file, output_dir, json_file = "german_test_video.json")