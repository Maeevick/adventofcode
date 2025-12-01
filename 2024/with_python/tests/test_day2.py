from src.day2 import part1, part2
from src.read_input import read_input

def test_day2_part1_says():
    assert part1(read_input("day2sample")) == 2


def test_day2_part2_says():
    assert part2(read_input("day2sample")) == 4
