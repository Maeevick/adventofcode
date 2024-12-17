defmodule Day2Test do
  use ExUnit.Case
  doctest Day2

  import ReadInput

  test "part1..." do
    assert Day2.part1(read_input("day2sample")) == 2
  end

  test "part2..." do
    assert Day2.part2(read_input("day2sample")) == 4
  end
end
