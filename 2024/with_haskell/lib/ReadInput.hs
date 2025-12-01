module ReadInput where

import System.FilePath ((</>))

readInput :: String -> IO [String]
readInput filename = lines <$> readFile (".." </> "inputs" </> filename <> ".txt")