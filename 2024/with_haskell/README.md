# Think Stupid

## Run the code

```shell
cabal run
```

## Run the test suite

```shell
cabal test
```

## HOW TO (_it's more for me than real guidelines for anyone else_)

- Add one file `DayX` in the `lib` folder

```haskell
module DayX where

part1 :: SomeType
part1 = "This is day XXX, part one"

part2 :: AnotherType
part2 = "This is day XXX, part two"
```

- Expose the `DayX` module in the library definition in the cabal file (_`exposed-modules`_)

```cabal
library advent-of-code-with-haskell-lib
    exposed-modules:    Hello, DayX
    hs-source-dirs:     lib
    build-depends:      base ^>=4.17.2.1
    default-language:   Haskell2010
```

- Add one file `DaySpecX` in the `test` folder
  - export `spec` for `hspec-discovery`
  - write your tests (I use the _`describe`_ for my test suite, the _`context`_ for each scenario / part and as many _`it`_ as necessary to find the solution )

```haskell
module Day1Spec(spec) where

import Test.Hspec

import Day1 (part1, part2)

spec :: Spec
spec = describe "dayX ..." $ do
  context "part 1..." $ do
    it "says ..." $ do
      part1 `shouldBe` "This is day XXX, part one"
  context "part2 ..." $ do
    it "says ..." $ do
      part2 `shouldBe` "This is day XXX, part two"
```
