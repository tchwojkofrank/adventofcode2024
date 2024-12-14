# Day 14: Restroom Redoubt

## Intro

Robots and their vectors.

## Part 1

A list of robots with 2d positions, and their velocities.
The grid wraps, so arithmetic is mod the size of the space.
I forgot to deal with negative positions/velocities, so my mod didn't work as expected.
After I sorted that out, it was all happy.

Though weirdly slow.

## Part 2

AH HA HA HA HA!

I didn't implement a test for part 2.
I did figure out the interval that is worth displaying the current state of things, and just watched it change over time to visually get the answer.

And then of course my first answer was off by one.

## Other notes

My Point2D<T> implementation in my Advent library is terrible.
Using native tuples was 10 times faster. I think I know why though. So at some point (ha!) I'll fix it.

## ChatGPT

