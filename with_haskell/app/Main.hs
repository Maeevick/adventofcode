module Main where

import Hello (hello)
import Day1 (part1, part2)

main :: IO ()
main = do
    putStrLn ("\n" ++ hello ++ "\n\n")
    putStrLn ("\n" ++ part1 ++ "\n\n")
    putStrLn ("\n" ++ part2 ++ "\n\n")
