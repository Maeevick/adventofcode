module Day4 (part1, part2) where
import qualified Data.Set as Set
import Control.Arrow ((&&&))

type Position = (Int, Int)
type Direction = (Int, Int)

directions :: Set.Set Direction
directions = Set.fromList [
    (-1,-1), (-1,0), (-1,1),
    (0,-1),          (0,1),
    (1,-1),  (1,0),  (1,1)]

getGridSize :: [String] -> (Int, Int)
getGridSize = length &&& length . head

isValidPosition :: [String] -> Position -> Bool
isValidPosition grid (x, y) = 
    x >= 0 && x < h && y >= 0 && y < w
    where
        (h, w) = getGridSize grid

getAllPositions :: [String] -> [Position]
getAllPositions grid = 
    [(x,y) | x <- [0..h-1],  y <- [0..w-1]]
    where
        (h, w) = getGridSize grid

getCharAtPosition :: [String] -> Position -> Maybe Char
getCharAtPosition grid (x, y)
    | isValidPosition grid (x, y) = Just ((grid !! x) !! y)
    | otherwise                   = Nothing

checkXMASPattern :: [String] -> Position -> Direction -> Bool
checkXMASPattern grid (x, y) (hDir, vDir) = and $ zipWith (==) (map Just "XMAS") $ [getCharAtPosition grid (x + i*hDir, y + i*vDir) | i <- [0..]]

part1 :: [String] -> Int
part1 content = length [(position, direction) | position <- getAllPositions content, getCharAtPosition content position == Just 'X', direction <- Set.toList directions, checkXMASPattern content position direction]


patterns :: Set.Set String
patterns = Set.fromList [
    "MMSS",
    "SSMM",
    "MSMS",
    "SMSM"]

getSymbolsAround :: [String] -> Position -> Maybe String
getSymbolsAround grid (x, y)
  | [Just tl, Just bl, Just tr, Just br] <- chars = Just [tl, bl, tr, br]
    | otherwise = Nothing
    where
        positions = [
            (x-1, y-1),  -- top-left
            (x+1, y-1),  -- bottom-left
            (x-1, y+1),  -- top-right
            (x+1, y+1)]  -- bottom-right
        chars = [getCharAtPosition grid pos | pos <- positions]


part2 :: [String] -> Int
part2 content = length 
  [(x,y) |
    x <- [1..h-2], 
    y <- [1..w-2], 
    getCharAtPosition content (x,y) == Just 'A', 
    Just symbols <- [getSymbolsAround content (x,y)], 
    symbols `Set.member` patterns]
  where (h, w) = getGridSize content
