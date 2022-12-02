import sys
import os
from pathlib import Path
import requests
import shutil


def get_input(day: int, year: int = 2022) -> str:
    cookie = os.environ["AOC_SESSION"]

    return requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": cookie}
    ).text


def main():
    day = sys.argv[1]
    # Create day file
    shutil.copy2(Path(f"src/solutions/template.rs"), Path(f"src/solutions/day{day}.rs"))

    # Attach day
    mod_path = Path(f"src/solutions/mod.rs")
    text = mod_path.read_text()
    text = text.replace("// End imports", f"mod day{day};\n// End imports ")
    text = text.replace(
        "d => ", f"{day} => day{day}::Problem {{}}.solve(raw_input),\n        d => "
    )
    mod_path.write_text(text)

    # Get input
    day_input = get_input(day)
    input_path = Path(f"inputs/day{day}.txt")
    input_path.write_text(day_input)


if __name__ == "__main__":
    main()
