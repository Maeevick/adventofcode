from collections import Counter


def part1(content: list[str]) -> int:
    left_numbers, right_numbers = zip(*(map(int, entry.split()) for entry in content))
    return sum(
        abs(left - right)
        for left, right in zip(sorted(left_numbers), sorted(right_numbers))
    )


def part2(content: list[str]) -> int:
    left_numbers, right_numbers = zip(*(map(int, entry.split()) for entry in content))

    return sum(left * Counter(right_numbers)[left] for left in left_numbers)
