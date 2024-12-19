from src.day3 import part1, part2
from src.read_input import read_input

def test_day3_part1_says():
    assert part1(read_input("day3part1sample")) == 161


def test_day3_part2_says():
    assert part2(read_input("day3part2sample")) == 48
