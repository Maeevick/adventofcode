defmodule Day1 do
  def part1(content) do
    {lefts, rights} = from_strings_to_pairs(content)

    Enum.zip(
      Enum.sort(lefts),
      Enum.sort(rights)
    )
    |> Enum.map(fn {left, right} -> abs(left - right) end)
    |> Enum.sum()
  end

  def part2(content) do
    {lefts, rights} = from_strings_to_pairs(content)
    occurrences = Enum.frequencies(rights)

    lefts
    |> Enum.map(fn left -> left * Map.get(occurrences, left, 0) end)
    |> Enum.sum()
  end

  defp from_strings_to_pairs(content) do
    content
    |> Enum.map(fn x -> String.split(x) end)
    |> List.flatten()
    |> Enum.map(fn x -> String.to_integer(x) end)
    |> Enum.chunk_every(2)
    |> Enum.map(fn [a, b] -> {a, b} end)
    |> Enum.unzip()
  end
end
