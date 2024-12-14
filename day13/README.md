# Day 13: Claw Contraption

## Intro

Linear algebra and THE CLAW

## Part 1

### Some simple math
Let's walk through the algebra (I know, that can be scary, but it'll be ok) for one machine, and then we'll go through the general (math) solution. This can also be done with linear algebra, but I that might be an abstraction too far for some, and probably overkill for this problem. Maybe another time well go into the matrix...

Here's the machine from the example:
```
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
```

So how many times do we press each button to get the prize?
The tricky bit here is that `X` and `Y` are NOT THE MATH VARIABLES! Shocking! Tricksy! Algebra not using `X` and `Y`!

The two variables are how many times you press A, and how many times you press B.
Let's call those to variables `A` and `B`. (Unimaginative, boring, but I hope clear.)
So the prize is at the coordinates (8400, 5400).
Each time you press A you add the vector (94,34).
Each time you press B you add the vector (22,67).
Vector addition and is commutative, so it doesn't matter which order you press the buttons, which simplifies things a bunch.
We end up with this equation:
```
A*(94,34) + B(22,67) = (8400,5400)
```
We can separate this out into two equations, one for each coordinate:
```
94A + 22B = 8400
34A + 67B = 5400
```
Two equations, two variables...algebra.
Let's solve the first equation for A:
```
94A + 22B = 8400
94A = 8400 - 22B (subtract 22B from both sides)
A = (8400 - 22B)/94 (divide both sides by 94)
```
Now let's subtitute this value for A in the second equation:
```
34A + 67B = 5400
34(8400 - 22B)/94 + 67B = 5400 (substitution for A)
34(8400 - 22B) + 94*67B = 94*5400 (multiply both sides by 94)
34*8400 - 34*22B + 94*67B = 94*5400 (distribution of 34)
94*67B - 34*22B + 34*8400 = 94*5400 (rearrange using commutation)
(94*67 - 34*22)B + 34*8400 = 94*5400 (factoring out the B)
(94*67 - 34*22)B = 94*5400 - 34*8400 (subtract 34*8400 from both sides)
B = (94*5400 - 34*8400)/(94*67 - 34*22) (divide both sides by (94*67 - 34*22))
B = 222000/5550 = 40
```
Now we can subsitute B in the equation for A:
```
A = (8400 - 22B)/94
A = (8400 - 22*40)/94
A = 80
```
So hit A 80 times and B 40 times, and you get the prize!
Generalizing this is just a matter of giving names to the values we were given.
So let's say:
For `Button A: X+94, Y+34` we choose `xa` for `94` and `ya` for 34.
For `Button B: X+22, Y+67` we choose `xb` for `22` and `yb` for 67.
For `Prize: X=8400, Y=5400` we choose `px` for `8400` and `py` for `5400`.
Doing the same steps as before we would get these two equations:
```
A = (px - xb*B)/xa
B = (xa*py - ya*px)/(xa*yb - ya*xb)
```

You can do the reverse, solving for B first and then getting an equation for A, or using linear algebra, and you'll get the equations I used in my answer, but that would really just be a different mechanism of doing the above. It's all moving things around on either side of the `=` sign.

## Part 2

It's the same as part 1, just really big numbers. Rust has 128 bit integers so, really no change.

## Other notes



## ChatGPT

