# Lefortovo

I use this to quickly fetch `.gitignore` and `Makefile` files from GitHub. The gitignore files are from GitHub's repository, the Makefiles are from my own https://github.com/bitemyapp/makefiles.git

## Installation

```
$ git clone git@github.com:bitemyapp/lefortovo.git
$ cd lefortovo && cargo install
```

## Use

To get a gitignore:


I usually inspect what I'm fetching first:

```
$ lefortovo --lang Haskell
```

Then I redirect the output on a re-run to the desired location:

```
$ lefortovo --lang Haskell > .gitignore
```

You can specify with `--gitignore` or `-g` but it defaults to fetching gitignore files anyway.

For fetching Makefiles:

```
$ lefortovo -m --lang Haskell
$ lefortovo -m --lang Haskell > Makefile
```

