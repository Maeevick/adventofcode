module Main where

import Day1 (part1, part2)
import Hello (hello)
import ReadInput (readInput)

main :: IO ()
main = do
  putStrLn "----------------"
  putStrLn hello
  putStrLn "----------------"
  putStrLn . ("D1P1: " <>) . show . Day1.part1 =<< readInput "day1part1"
  putStrLn . ("D1P2: " <>) . show . Day1.part2 =<< readInput "day1part1"
  putStrLn "----------------"