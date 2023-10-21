# aors - Advent of RS 🎄 🦀

Useful rs tools for Advent of Code.

## Installation & Setup

```sh
cargo install aors      # Install CLI
aors --init             # Create a new project at current directory.

aors --year <YEAR>      # Set year to solve
aors --cookie <COOKIE>  # Set session cookie
```

COOKIE is your session cookie, acquired from the advent of code website [like this](https://github.com/wimglenn/advent-of-code-wim/issues/1).

There is a risk that the dependency is not added correctly to the project. If so, just add it manually in `Cargo.toml`

The first command after this can take some time without output. This is completely normal.

## Usage

```sh
aors            # run all days
aors [DAYS]     # run one or more specific days
aors [DAYS] -x  # run with example input (--example also works)

aors [DAYS] -g  # get input and create necessary files (--get also works)
                # input_examples/<DAY>.in has to be filled in manually

aors [DAYS] -p  # post output to Advent of Code (--post also works)
                # submits part 2 if part 1 has already been submitted
```

## The `phi()` function

The puzzles sometimes require that different variables be used for the example problem and the actual problem, beyond just the input values. The built-in function `phi()` is used to handle such situations.

`phi()` takes two inputs `actual` and `example` of the same type. It returns `actual` when solving the actual problem and `example` when using example input.

## Helper functions

If a function is needed in multiple days, it should be placed in the `helpers` subcrate.

##

Initial project structure and setup kindly stolen from [AxlLind](https://github.com/AxlLind).
However, it has been greatly expanded upon since.
