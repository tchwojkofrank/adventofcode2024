# Day 0: TBD

## Intro

Stone proliferation per blink.

## Part 1

How many stones do you have at any given moment if:
0 -> 1
xx -> x x where x is some number of digits
y -> y*2024 where y is any other case

Straight up simulated for 25 "blinks". Not too bad.

## Part 2

Now simulate for 75 "blinks". Hit a wall after 30-40 blinks.
Instead of simulating each stone individually, just keep track of how many of each stone you have at any given step.
If I have N "0" stones, then after the next blink I will have N "1" stones.
The number of unique stone labels don't explode, so each step is manageable.

## Other notes

This is probably not at all efficient Rust since I'm creating a new HashMap each iteration.

## ChatGPT

