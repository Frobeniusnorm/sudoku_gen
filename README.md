# sudoku_gen
Generates Sudokus in various logic levels:
- Logic Level 1: only numbers that arethe only possible ones for the fields are inserted
- Logic Level 2: Additionally numbers are inserted in fields if the field is the only possibility for the number in the group (row / column / block)
- Logic Level 3: speculative insertion. If in a field a set of numbers are possible and inserting each of the numbers leads to another field `k` to become number `n`, `k` can be set to `n`

Is able of generating the solution to the problem and output it to the terminal or an image (both).
Assigns a score to each problem depending on the simplest solution that can be found (difficulty increases with logic level, level 1 has always difficulty 1, for the other levels difficulty sums up).

Needs a font file to generate images, this repository contains [Fira Code Mono](https://github.com/tonsky/FiraCode).
