import json


def parse_date(transcript_name):
    """
    Parses a transcript file, extracts speech segments with start and end times,
    and saves the data as JSON.

    Args:
        transcript_name (str): Name of the transcript file (without extension).
        output_filename (str, optional): Name of the output JSON file. Defaults to "transcript_data.json".
    """
    from IPython import embed
    transcript_name = transcript_name.split(".")[0]
    parsed_data = []
    with open(f"{transcript_name}.txt", "r") as file:
        lines = list(filter(None, file.read().split("\n")))

    for i in range(len(lines) - 1):
        embed()
        init_time, phrase = lines[i].split("s: ")
        next_init_time, _ = lines[i + 1].split("s: ")
        parsed_data.append(
            {
                "init_time": init_time.strip(),
                "final_time": next_init_time.strip(),
                "phrase": phrase.strip(),
            }
        )

    init_time, phrase = lines[-1].split("s: ")
    parsed_data.append(
        {"init_time": init_time.strip(), "final_time": None, "phrase": phrase.strip()}
    )

    with open(f"{transcript_name}.json", "w") as outfile:
        json.dump(parsed_data, outfile, indent=4)  # Add indentation for readability
