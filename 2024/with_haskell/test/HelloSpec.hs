module HelloSpec (spec) where

import Hello (hello)
import Test.Hspec

spec :: Spec
spec = describe "setup is ok" $ do
  it "says hello" $ do
    hello `shouldBe` "Hello, advent of code with haskell!"
