defmodule HelloTest do
  use ExUnit.Case
  doctest Hello

  test "setup is ok" do
    assert Hello.hello() == "Hello, advent of code with elixir!"
  end
end
