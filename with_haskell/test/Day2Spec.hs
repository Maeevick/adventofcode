module Day2Spec (spec) where

import Day2 (part1, part2)
import ReadInput (readInput)
import Test.Hspec

spec :: Spec
spec = do
  describe "part 1 with sample data" $ do
    it "returns 2" $ do
      content <- readInput "day1sample"
      part1 content `shouldBe` 2
  describe "part2 with sample data" $ do
    it "returns 4" $ do
      content <- readInput "day2sample"
      part2 content `shouldBe` 4
