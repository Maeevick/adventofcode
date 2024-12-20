module Day3Spec (spec) where

import Day3 (part1, part2)
import ReadInput (readInput)
import Test.Hspec

spec :: Spec
spec = do
  describe "part 1 with sample data" $ do
    it "returns 161 (2*4 + 5*5 + 11*8 + 8*5)" $ do
      content <- readInput "day3part1sample"
      part1 content `shouldBe` 161
  describe "part2 with sample data" $ do
    it "returns 48 (2*4 + 8*5)" $ do
      content <- readInput "day3part2sample"
      part2 content `shouldBe` 48
