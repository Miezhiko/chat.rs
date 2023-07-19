{-# LANGUAGE UnicodeSyntax #-}

import           Hake

main ∷ IO ()
main = hake $ do

  "clean | clean the project" ∫
    cargo ["clean"] ?> removeDirIfExists targetPath

  "update | update dependencies" ∫ cargo ["update"]

  chatLib ♯ cargo ["build"]

  "install | install to system" ∫
    cargo ["install", "--release"]

 where
  targetPath ∷ FilePath
  targetPath = "target"

  buildPath ∷ FilePath
  buildPath = targetPath </> "debug"

  chatLib ∷ FilePath
  chatLib = buildPath </> "libchat.rlib"
