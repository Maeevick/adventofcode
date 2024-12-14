from hello import hello
from day1 import part1 as day1part1, part2 as day1part2
from readInput import readInput


def main():
    print("----------------")
    print(f"{hello()}")
    print("----------------")
    print(f"D1P1: {day1part1(readInput("day1part1"))}")
    print(f"D1P2: {day1part2(readInput("day1part1"))}")
    print("----------------")


if __name__ == "__main__":
    main()
