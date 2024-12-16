defmodule AdventOfCodeWithElixir do
  def start() do
    IO.puts("----------------")
    IO.puts(Hello.hello())
    IO.puts("----------------")
    IO.puts("D1P1: #{Day1.part1(ReadInput.read_input("day1part1"))}")
    IO.puts("D1P2: #{Day1.part2(ReadInput.read_input("day1part1"))}")
    IO.puts("----------------")
  end
end

AdventOfCodeWithElixir.start()
