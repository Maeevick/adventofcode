module Day2 (part1, part2) where
import Data.List (sort, zip, tail, sortBy)
import qualified Data.Ord
import Data.Ord (comparing)

part1 :: [String] -> Int
part1 = foldl checkValidity 0
  where
    checkValidity acc = (+) acc . fromEnum . isValid . map read . words
    isValid xs = isSorted xs && isGapOk xs

part2 :: [String] -> Int
part2 = foldl checkValidity 0
  where
    checkValidity acc = (+) acc . fromEnum . isValid . map read . words
    isValid xs = (isSorted xs && isGapOk xs) || isValidAllowingOneException xs

    isValidAllowingOneException :: [Int] -> Bool
    isValidAllowingOneException xs = any (\x -> isSorted x && isGapOk x) (removeOneAt xs)


isSorted :: [Int] -> Bool
isSorted xs = xs == sort xs || xs == sortBy (comparing Data.Ord.Down) xs

isGapOk :: [Int] -> Bool
isGapOk xs = all (\(x, y) -> abs (x - y) >= 1 && abs (x - y) <= 3) (zip xs (tail xs))

removeOneAt :: [a] -> [[a]]
removeOneAt [] = []
removeOneAt (x:xs) = xs : map (x:) (removeOneAt xs)