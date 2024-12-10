# Day 9: Disk Fragmenter

## Intro

Moving run length encoded blocks around. (Trivia: A Lotus screenshot tool for either DOS or Windows used an alternating run length encoding format: A one byte repeat count, a value, a one byte count N, N values, repeat...

## Part 1

The first part is to move parts of blocks into the empty spaces according to a specific ordering, and do some math. Just doing what it said worked fine.

I made the math 64 bit to try to avoid overflows in the result, which multiplied things.

## Part 2

The second part was to keep blocks contiguous. I misread the problem and made it much more difficult (and got the wrong answer). The actual answer required a lot less code, and a bit more awake-ness.

## Other notes



## ChatGPT

### Part 1
ChatGPT is taking a long time to run the program.
It decided that it needs to optimize.
It got the wrong answer, and I told it its answer was too high.
It got an answer that was too low.
It then gave an answer that was higher than the first time it tried to solve it.

### Part 2
I gave up and gave it part 2.
It got this part correct without any problem. ¯\_(ツ)_/¯