module Main where

import Day1 (part1, part2)
import Day2 (part1, part2)
import Day3 (part1, part2)
import Hello (hello)
import ReadInput (readInput)

main :: IO ()
main = do
  putStrLn "----------------"
  putStrLn hello
  putStrLn "----------------"
  putStrLn . ("D1P1: " <>) . show . Day1.part1 =<< readInput "day1"
  putStrLn . ("D1P2: " <>) . show . Day1.part2 =<< readInput "day1"
  putStrLn "----------------"
  putStrLn . ("D2P1: " <>) . show . Day2.part1 =<< readInput "day2"
  putStrLn . ("D2P2: " <>) . show . Day2.part2 =<< readInput "day2"
  putStrLn "----------------"
  putStrLn . ("D3P1: " <>) . show . Day3.part1 =<< readInput "day3"
  putStrLn . ("D3P2: " <>) . show . Day3.part2 =<< readInput "day3"
  putStrLn "----------------"
