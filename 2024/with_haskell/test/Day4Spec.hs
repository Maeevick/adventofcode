module Day4Spec (spec) where

import Day4 (part1, part2)
import ReadInput (readInput)
import Test.Hspec

spec :: Spec
spec = do
  describe "part 1 with sample data" $ do
    it "says..." $ do
      content <- readInput "day4sample"
      part1 content `shouldBe` 18
  describe "part2 with sample data" $ do
    it "says..." $ do
      content <- readInput "day4sample"
      part2 content `shouldBe` result
