from src.day1 import part1, part2
from src.readInput import readInput


def test_day1_part1_says():
    assert part1(readInput("day1part1sample")) == 11


def test_day1_part2_says():
    assert part2(readInput("day1part1sample")) == 31
