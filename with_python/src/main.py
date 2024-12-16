from hello import hello
from day1 import part1 as day1part1, part2 as day1part2
from read_input import read_input


def main():
    print("----------------")
    print(f"{hello()}")
    print("----------------")
    print(f"D1P1: {day1part1(read_input("day1part1"))}")
    print(f"D1P2: {day1part2(read_input("day1part1"))}")
    print("----------------")


if __name__ == "__main__":
    main()
