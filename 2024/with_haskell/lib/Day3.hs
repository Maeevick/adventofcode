module Day3 (part1, part2) where

import Text.Regex.TDFA ((=~))
import Data.Maybe (fromMaybe)

part1 :: [String] -> Int
part1 = sum . extractMuls . concat
  where
    extractMuls :: String -> [Int]
    extractMuls input =
        [ read a * read b
        | [_, a, b] <- input =~ "mul\\(([0-9]{1,3}),([0-9]{1,3})\\)" :: [[String]]
        ]

part2 :: [String] -> Int
part2 = fst . foldl processMatch (0, True) . findMatches . concat
  where
    findMatches :: String -> [[String]]
    findMatches t = t =~ "(do|don't)\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)" :: [[String]]

    processMatch :: (Int, Bool) -> [String] -> (Int, Bool)
    processMatch (sum, _) [_, "do", "", ""] = (sum, True)
    processMatch (sum, _) [_, "don't", "", ""] = (sum, False)
    processMatch (sum, True) [_, "", a, b] = (sum + read a * read b, True)
    processMatch state _ = state