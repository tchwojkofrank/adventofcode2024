# adventofcode2024

## Spoiler warning
I'll document each day in a readme file, so don't go delving into the subdirectories if you want to avoid spoilers.

## A little Rust-y
I'll be working in Rust this year. As usual, the code will be quick and dirty.

## ChatGPT test
After I complete a day, I'll also test ChatGPT a bit with the raw wording, and then my rewording, and see how it does.

## Setting up a day
Run `./startday.sh xx` where `xx` is a day number to set up a directory for the day.
That will set up the directory structure, run `cargo init`, copy the relevant bits from `day00`, and build and run tests to make sure it was set up correctly.

## Tests
The default input file is `files/test`, and the default expected answers for parts 1 and 2 of each day's problem are `files/test_answer1` and `files/test_answer2`. The default test reads the input file and checks the result against the output file.

## The `advent` library
This library has the basic conveniences for reading and splitting input. I'll add other things there as needed (such as point structures, vector math, data structure algorithms, etc.).

# Results

## Final tally:

44 ⭐️
* 2 days did not attempt (Day 21 and 25)
* Day 21 didn’t know how to start
* Day 25 didn’t really look at it.
* 2 days only part 1
* Day 17, due to a bug or my approach is off.
* Day 24, didn’t get to it, but have an approach.

After some peeking at other solutions, I figured out I was on the right track for Day 17, but was overthinking it, and had bugs.

