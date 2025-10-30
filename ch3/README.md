# Mimic GNU cat

## Test
- In `run`, you iterate with `for line in file.lines()` and print with `println!`.
- `BufRead::lines()` strips the newline terminator; `println!` always adds one back, even for the final line with no terminator in the source.
- GNU `cat` preserves the exact newline semantics: if the input’s last line has no trailing `\n`, `cat` prints the last line without a newline. That is what your `mk-outs.sh` generated for the stdin expected files:
    - `tests/expected/the-bustle.txt.n.stdin.out`
    - `tests/expected/the-bustle.txt.b.stdin.out`
- Your non-stdin test `bustle_stdin` passes only because the expected file `tests/expected/the-bustle.out.txt` was manually created with a final blank line to match `println!` behavior.
