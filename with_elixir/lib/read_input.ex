defmodule ReadInput do
  def read_input(filename) do
    Path.join(["..", "inputs", "#{filename}.txt"])
    |> File.read!()
    |> String.split(~r/\r?\n/)
  end
end
