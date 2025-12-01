defmodule Day2 do
  def part1(content) do
    from_report_to_int(content)
    |> Enum.count(fn x -> is_valid?(x) end)
  end

  def part2(content) do
    from_report_to_int(content)
    |> Enum.count(fn x -> is_valid?(x) || is_valid_allowing_one_exception?(x) end)
  end

  defp from_report_to_int(content) do
    content
    |> Enum.map(fn x -> String.split(x) |> Enum.map(&String.to_integer/1) end)
  end

  defp is_valid?(report) do
    is_sorted?(report) and is_gap_ok?(report)
  end

  defp is_sorted?(report) do
    report == Enum.sort(report) || report == Enum.sort(report, :desc)
  end

  defp is_gap_ok?(report) do
    report
    |> Enum.zip(tl(report))
    |> Enum.all?(fn {a, b} -> abs(a - b) >= 1 and abs(a - b) <= 3 end)
  end

  defp is_valid_allowing_one_exception?(report) do
    report
    |> remove_one()
    |> Enum.any?(fn xs -> is_sorted?(xs) and is_gap_ok?(xs) end)
  end

  defp remove_one([]), do: []

  defp remove_one([x | xs]) do
    [xs | Enum.map(remove_one(xs), &[x | &1])]
  end
end
