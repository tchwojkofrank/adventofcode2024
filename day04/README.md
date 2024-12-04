# Day 4: Ceres Search

## Part 1

Word search for `XMAS`
Get the grid, at each location search in each direction for `XMAS`.

## Part 2

Find all the diagonal crosses of `MAS`.
Get the grid, at each location check for `A`, and then check the two diagonals to make sure they fit the pattern. I could have done something tricky to map the letters to numbers, and do some arithmetic on the diagonals, but nested `if` statements were just quicker, easier, and safer.

## ChatGPT

Solved part 1 quickly, but needed more explanation of part 2. I should have just told it it was wrong and seen what happened. Instead I explained where it misunderstood the instructions.