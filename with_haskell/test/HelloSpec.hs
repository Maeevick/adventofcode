module HelloSpec(spec) where

import Test.Hspec

import Hello (hello)

spec :: Spec
spec = describe "setup is ok" $ do
  it "says hello" $ do
    hello `shouldBe` "Hello, advent of code with haskell!"
