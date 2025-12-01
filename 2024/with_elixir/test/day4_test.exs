defmodule Day4Test do
  use ExUnit.Case
  doctest Day4

  import ReadInput

  test "part1..." do
    assert Day4.part1(read_input("day4sample")) == 18
  end

  test "part2..." do
    assert Day4.part2(read_input("day4sample")) == 9
  end
end
