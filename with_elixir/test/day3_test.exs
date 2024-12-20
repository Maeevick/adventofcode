defmodule Day3Test do
  use ExUnit.Case
  doctest Day3

  import ReadInput

  test "part1..." do
    assert Day3.part1(read_input("day3part1sample")) == 161
  end

  test "part2..." do
    assert Day3.part2(read_input("day3part2sample")) == 48
  end
end
