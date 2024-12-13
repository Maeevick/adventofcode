module Day1Spec(spec) where

import Test.Hspec

import Day1 (part1, part2)

spec :: Spec
spec = describe "day1 ..." $ do
  context "part 1..." $ do
    it "says ..." $ do
      part1 `shouldBe` "This is day one, part one"
  context "part2 ..." $ do
    it "says ..." $ do
      part2 `shouldBe` "This is day one, part two"