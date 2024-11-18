# adventofcode2024

## A little Rust-y
I'll be working in Rust this year. As usual, the code will be quick and dirty.

## Setting up a day
Run `./startday.sh xx` where `xx` is a day number to set up a directory for the day.
That will set up the directory structure, run `cargo init`, copy the relevant bits from `day00`, and build and run tests to make sure it was set up correctly.

## Tests
The default input file is `files/test`, and the default expected answers for parts 1 and 2 of each day's problem are `files/test_answer1` and `files/test_answer2`. The default test reads the input file and checks the result against the output file.

## The `advent` library
This library has the basic conveniences for reading and splitting input. I'll add other things there as needed (such as point structures, vector math, data structure algorithms, etc.).
