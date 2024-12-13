defmodule AdventOfCodeWithElixir do
  def start(str) do
    IO.puts("\n\n" <> str <> "\n")
  end
end

AdventOfCodeWithElixir.start(Hello.hello())
AdventOfCodeWithElixir.start(Day1.part1())
AdventOfCodeWithElixir.start(Day1.part2())
