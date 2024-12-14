module Day1 (part1, part2) where

import Control.Arrow (Arrow ((***)))
import Data.List (sort)

part1 :: [String] -> Int
part1 = sum . uncurry (zipWith (\x y -> abs (x - y))) . (sort *** sort) . fromListToListOfTuple

part2 :: [String] -> Int
part2 = sum . multiplyCount . fromListToListOfTuple
  where
    multiplyCount (lefts, rights) = map (\left -> left * length (filter (== left) rights)) lefts

fromListToListOfTuple :: [String] -> ([Int], [Int])
fromListToListOfTuple = unzip . map ((\[x, y] -> (read x, read y)) . words)