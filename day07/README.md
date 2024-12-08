# Day 7: Bridge Repair

## Intro

Calibrations and missing operators.

## Part 1

We are given a bunch of calibrations.
Each calibration is an answer followed by a list of operands. The operands can be `+` or `*`.
If there is a way to organize operators such that when applied to the operands in left to right order, then the calibration is valid.

So working backwards, if:
`answer = (...) + a_n`
then
`answer - a_n = (...)`.

Or if multiplying:
`answer / a_n = (...)`.

So:
Try subtraction, then recurse.
Try divison, then recurse.

I also check if the answer is positive after subtraction, or the remainder is zero for division, to help cut out of the recursion earlier.

## Part 2

They missed an operator: concatenation.
We just add another case in the recursion:
Check that `a_n` is a suffix of `answer`, and if it is, trim it, and recurse.

## ChatGPT

Didn't get this one.