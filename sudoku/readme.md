# Sudoku

This application will attempt to generate a [sudoku](https://en.wikipedia.org/wiki/Sudoku) puzzle and then solve it, writing the problem to `puzzle.txt` and answer to `solution.txt`.

## Generate a puzzle

Running this application will generate a solvable sudoku puzzle and write to `puzzle.txt`

```
cargo run
```

> No puzzle file found, generating...
> puzzle written to puzzle.txt

## Solving a puzzle

Running this appliction with a `puzzle.txt` file, the application will attempt to find a valid solution for the sudoku problem and write this to `solution.txt`.

```
cargo run
```

> Parsing puzzle.txt and solving...
> solution written to solution.txt
