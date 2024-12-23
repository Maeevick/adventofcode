defmodule Day4 do
  def part1(content) do
    content
    |> Enum.with_index()
    |> Enum.flat_map(fn {row, x} ->
      String.graphemes(row)
      |> Enum.with_index()
      |> Enum.filter(fn {char, _} -> char == "X" end)
      |> Enum.map(fn {_, y} -> {x, y} end)
    end)
    |> Enum.flat_map(fn {x, y} -> check_xmas_words(content, x, y) end)
    |> Enum.count()
  end

  def part2(content) do
    content
    |> Enum.with_index()
    |> Enum.drop(1)
    |> Enum.take(length(content) - 2)
    |> Enum.flat_map(fn {row, x} ->
      String.graphemes(row)
      |> Enum.with_index()
      |> Enum.drop(1)
      |> Enum.take(String.length(row) - 2)
      |> Enum.filter(fn {char, _y} -> char == "A" end)
      |> Enum.map(fn {_char, y} -> {x, y} end)
    end)
    |> Enum.count(fn {x, y} -> check_xmas_patterns(content, {x, y}) end)
  end

  defp at(grid, x, y) do
    if x >= 0 and y >= 0 and x < length(grid) and y < String.length(Enum.at(grid, 0)) do
      grid
      |> Enum.at(x)
      |> String.at(y)
    end
  end

  defp check_xmas_words(content, x, y) do
    [
      {-1, -1},
      {-1, 0},
      {-1, 1},
      {0, -1},
      {0, 1},
      {1, -1},
      {1, 0},
      {1, 1}
    ]
    |> Enum.filter(fn {dx, dy} ->
      "XMAS"
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.all?(fn {char, i} ->
        at(content, x + dx * i, y + dy * i) == char
      end)
    end)
  end

  defp check_xmas_patterns(content, {x, y}) do
    [
      at(content, x - 1, y - 1),
      at(content, x + 1, y - 1),
      at(content, x - 1, y + 1),
      at(content, x + 1, y + 1)
    ] in [
      ["M", "M", "S", "S"],
      ["S", "S", "M", "M"],
      ["M", "S", "M", "S"],
      ["S", "M", "S", "M"]
    ]
  end
end
