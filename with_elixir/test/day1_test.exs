defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  import ReadInput

  test "part1..." do
    assert Day1.part1(read_input("day1part1sample")) == 11
  end

  test "part2..." do
    assert Day1.part2(read_input("day1part1sample")) == 31
  end
end
