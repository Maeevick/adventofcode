from src.day4 import part1, part2
from src.read_input import read_input

def test_day4_part1_says():
    assert part1(read_input("day4sample")) == 18


def test_day4_part2_says():
    assert part2(read_input("day4sample")) == 9
