# Advent Of Code 2022

My solutions for [Advent Of Code](https://adventofcode.com) 2022.

## Usage

This project uses [`just`](https://github.com/casey/just) to speed up some commands. For some commands to work (like downloading puzzle input or submitting a solution) a session token needs to be provided in the `AOC_SESSION` environment variable. The easiest way to set it is to create the file `.env` with `AOC_SESSION=your token` inside in the root of this repository. The token can be received by reading the session cookie from the AOC website.

Download the puzzle input, create a file for the current day and open it in vscode:

```shell
just begin        # Prepare the current day
just day=09 begin # Prepare day 9
```

Run:

```shell
just        # Runs the current day
just day=09 # Runs day 9
```

After running a puzzle the output is saved to `output/XX.log`. This log will be read when submitting puzzles:

```shell
just submit 1        # Submit the first half of a puzzle
just submit 2        # Submit the second half of a puzzle
just day=09 submit 2 # Submit the second half of the puzzle for day 9
```

Submit looks for a line containing `level-1-solution=123` to submit 123 as the solution to level 1. There is a convenience function `solved_level_1` in `lib.rs` you can use to avoid printing this line manually.
