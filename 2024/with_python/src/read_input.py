import os


def read_input(filename: str) -> list[str]:
    path = os.path.join("..", "inputs", f"{filename}.txt")
    with open(path, "r", encoding="utf-8") as file:
        return file.read().splitlines()
