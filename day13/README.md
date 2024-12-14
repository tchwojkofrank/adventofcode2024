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

Ok, so the linear algebra. I find it magical, and boring, and interesting when it's more abstract.

Here is a terrible attempt at trying to explain it using just ASCII characters. (Using real math symbols and drawings would be much more effective.)

I'll stick with 2x2 things here to keep it small, but it'll generalize to NxN, and lots of alternating pluses and minuses everywhere, but the pattern is easily found elsewhere and easily memorized if you really have a desire to do this by hand.

So you have two equations:
```
Ax + By = C
Dx + Ey = F
```

You can write these things in a grid, and apply a pattern of multiplication and addition to it, so you get a 2x2 matrix and two (veritcal) 2-vectors.
```
| A  B |  |x|     |C|
| D  E |  |y|  =  |F|
```
Take the top row of the 2x2 matrix, [A,B], transpose it and take the dot product with [x,y]. That becomes the first (top) value on the left of the equation. Do the same with the bottom row.
You get back our system of equations:
```
| Ax + By |  =  |C|
| Dx + Ey |  =  |F|
```
You can multiply matrices too (if the dimensions match up), using the same transpose rule:
```
| A B | | C D |   |AC+BG  AD+BH|
| E F | | G H | = |EC+FG  ED+FH|
```
Just like with multiplication of numbers, there is the matrix equivalent of `1` called the identity, where if you multiply it by something, you just get the something. It looks like this:
```
|1 0|
|0 1|
```
Go ahead and try it with either a 2-vector or another 2x2 matrix.

Also just like regular numbers, matrices have inverses. We can use variables for entire matrices as well. I'll use `I` for the identity matrix.
So if `U` is a matrix and `V` is the inverse of `U`, then

```
UV = I and VU = I
```

A pattern that shows up in dealing with matrices is taking the difference of the products of the diagonals (it's more complicated with 3x3 matrices, but there are other places to go find that pattern). That difference of the product is called the `determinant` or `det A`.

```
    | A B |
det | C D | = AD - BC
```

You can do a bunch of algebra* and figure out that the inverse of the matrix will be:

```
| A B |-1      1    |  D -B |
|     |   = _______ |       |
| C D |      det A  | -C  A |
```
*I know you're likely going to just take my word for it. That's fine, but I encourage you to give it a try.

So we have the inverse for matrices, which is a lot like division, and we can do the obvious thing to solve our system of equations, using a little shorthand. Let's rename our pieces a bit.

Here's our original problem expressed as matrices and vectors:
```
| A  B |  |x|     |C|
| D  E |  |y|  =  |F|
```
Let's call the matrix A, the unknown (x,y) vector X, and the answer vector on the right, B.
So
```
AX = B
```
If we multiply both sides by the inverse matrix we get:
```
A^-1 A X = A^-1 B
```
Since `A^1` is the inverse of `A`, when we multiply them we get the identity matrix `I`
```
I X = A^-1 B
```
So
```
X = A^-1 B
```
and using our formula for the inverse of A
```
|x|      1    |  E  -B | |C|       1     |  E  -B | |C|
| | = _______ |        | | | = _________ |        | | |
|y|    det A  | -D   A | |F|    AE - BD  | -D   A | |F|
```
or
```
|x|       1     | EC-BF |
| | = _________ |       | 
|y|    AE - BD  | AF-CD |
```
or back to our regular looking algebra
```
x = (EC-BF)/(AE-BD)
y = (AF-CD)/(AE-BD)
```
Which is pretty much the same thing we got with our algebra.
In the end, matrix manipulation is just a mix of shorthand and shortcuts for doing the mechanics of lots of algebraic manipulations.

Keeping in mind this equivalence, you can see that you can do things like multiply the top row of both sides of the matrix equation by, say 7, and it doesn't change the result, because that's the equivalent of multiplying both sides of the first equation by 7.

Similarly adding the entire bottom row to to the top row (leaving the bottom row as it is) also doesn't change the result, because you're adding two different representations of the same thing to both sides of the top equation.

It's similar to how long division is a mechanical process that's guaranteed to work, by removing the tedium of doing lots of subtractions, and replacing it with an entirely different and more obscure tedium.

## ChatGPT

