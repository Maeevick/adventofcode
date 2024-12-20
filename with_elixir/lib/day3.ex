defmodule Day3 do
  def part1(content) do
    content
    |> Enum.join()
    |> process_mul()
  end

  def part2(content) do
    content
    |> Enum.join()
    |> process_conditional_mul()
  end

  defp process_mul(input) do
    Regex.scan(~r/mul\((\d{1,3}),(\d{1,3})\)/, input, capture: :all_but_first)
    |> Enum.map(fn [a, b] -> String.to_integer(a) * String.to_integer(b) end)
    |> Enum.sum()
  end

  defp process_conditional_mul(input) do
    Regex.scan(~r/(do|don't)\(\)|mul\((\d{1,3}),(\d{1,3})\)/, input, capture: :all_but_first)
    |> Enum.reduce({0, true}, fn
      ["do"], {sum, _} -> {sum, true}
      ["don't"], {sum, _} -> {sum, false}
      ["", a, b], {sum, true} -> {sum + String.to_integer(a) * String.to_integer(b), true}
      _, state -> state
    end)
    |> elem(0)
  end
end
