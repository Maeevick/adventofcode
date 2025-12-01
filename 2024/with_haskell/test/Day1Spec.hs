module Day1Spec (spec) where

import Day1 (part1, part2)
import ReadInput (readInput)
import Test.Hspec

spec :: Spec
spec = do
  describe "part 1 with sample data" $ do
    it "returns 11" $ do
      content <- readInput "day1sample"
      part1 content `shouldBe` 11
  describe "part2 with sample data" $ do
    it "returns 31" $ do
      content <- readInput "day1sample"
      part2 content `shouldBe` 31