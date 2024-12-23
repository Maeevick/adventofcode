from hello import hello
from day1 import part1 as day1part1, part2 as day1part2
from day2 import part1 as day2part1, part2 as day2part2
from day3 import part1 as day3part1, part2 as day3part2
from day4 import part1 as day4part1, part2 as day4part2
from read_input import read_input


def main():
    print("----------------")
    print(f"{hello()}")
    print("----------------")
    print(f"D1P1: {day1part1(read_input("day1"))}")
    print(f"D1P2: {day1part2(read_input("day1"))}")
    print("----------------")
    print(f"D2P1: {day2part1(read_input("day2"))}")
    print(f"D2P2: {day2part2(read_input("day2"))}")
    print("----------------")
    print(f"D3P1: {day3part1(read_input("day3"))}")
    print(f"D3P2: {day3part2(read_input("day3"))}")
    print("----------------")
    print(f"D4P1: {day4part1(read_input("day4"))}")
    print(f"D4P2: {day4part2(read_input("day4"))}")
    print("----------------")


if __name__ == "__main__":
    main()
