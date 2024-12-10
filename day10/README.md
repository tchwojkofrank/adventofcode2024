# Day 0: TBD

## Intro

A 2x2 grid of numbers, find paths (without diagonals) that go from 0 to 9, by 1 step increments.
0 is the trailhead.
9 is the peak.

## Part 1

Find the number of peaks reachable from each trailhead.
Sum for all trailheads.
A little recursion, checking north, east, south, and west directions for the next step in the path.
If a path reaches a peak, add it to a HashSet.
Sum the number of peaks for each trailhead.
I also had the number of ways to reach the peak from each trailhead, but didn't use it.

## Part 2

Oh, for part 2 I use the number of paths instead of the number of peaks.
Well that was quick.

## Other notes



## ChatGPT

