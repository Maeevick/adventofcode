defmodule AdventOfCodeWithElixir do
  def start() do
    IO.puts("----------------")
    IO.puts(Hello.hello())
    IO.puts("----------------")
    IO.puts("D1P1: #{Day1.part1(ReadInput.read_input("day1"))}")
    IO.puts("D1P2: #{Day1.part2(ReadInput.read_input("day1"))}")
    IO.puts("----------------")
    IO.puts("D2P1: #{Day2.part1(ReadInput.read_input("day2"))}")
    IO.puts("D2P2: #{Day2.part2(ReadInput.read_input("day2"))}")
    IO.puts("----------------")
    IO.puts("D3P1: #{Day3.part1(ReadInput.read_input("day3"))}")
    IO.puts("D3P2: #{Day3.part2(ReadInput.read_input("day3"))}")
    IO.puts("----------------")
  end
end

AdventOfCodeWithElixir.start()
