from src.day1 import part1, part2
from src.read_input import read_input


def test_day1_part1_says():
    assert part1(read_input("day1part1sample")) == 11


def test_day1_part2_says():
    assert part2(read_input("day1part1sample")) == 31
