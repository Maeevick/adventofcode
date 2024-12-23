import re


def part1(content: list[str]) -> int:
    matches = re.finditer(r"mul\((\d{1,3}),(\d{1,3})\)", "".join(content))
    return sum(int(a) * int(b) for match in matches for a, b in [match.groups()])


def part2(content: list[str]) -> int:
    matches = re.finditer(
        r"(don't|do)\(\)|mul\((\d{1,3}),(\d{1,3})\)", "".join(content)
    )
    processing = True
    sum = 0

    for match in matches:
        instruction, a, b = match.group(1), match.group(2), match.group(3)

        if instruction in ("do", "don't"):
            processing = instruction == "do"

        if processing and a and b:
            sum += int(a) * int(b)

    return sum
