# Think Stupid

## Run the code

```shell
mix compile
mix run lib/main.ex
```

## Run the test suite

```shell
mix test
```

## Run the formatter

```shell
mix format
```

## HOW TO (_it's more for me than real guidelines for anyone else_)

- Add one file `dayX` in the `lib` folder

```elixir
defmodule Day1 do
  def part1 do
    "This is day one, part one"
  end

  def part2 do
    "This is day one, part two"
  end
end
```

- Add one file `day_test.exs` in the `test` folder

```elixir
defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "part1..." do
    assert Day1.part1() == "This is day one, part one"
  end

  test "part2..." do
    assert Day1.part2() == "This is day one, part two"
  end
end
```
