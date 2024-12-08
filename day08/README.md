# Day 8: Resonant Collinearity

## Intro

A grid of antennas with various labels.

## Part 1

The goal is to group all the antennas by label, and then for any pair of antennas with the same label at points P1 and P2,
find the point P2 + (P2-P1) and P1 - (P2-P1). Count how many are within the map.

I used a HashMap of chars -> Vector of Points to group antennas, then double looped over the Vector for each label to find the points.

## Part 2

Same thing, but allow multiples:
P2 + n(P2-P1)
and
P1 - n(P2-P1)
for all possible n, that stay on the grid.
Same as before, but instead of adding or subtracting the difference, do it repeatedly. Also inlude the antennas themselves (if there is more than one with that label).

## Other notes

 

## ChatGPT

