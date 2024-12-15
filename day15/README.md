# Day 15: Warehouse Woes

## Intro

Robots pushing boxes

## Part 1

We have the map, and the list of moves for the robot.
Before moving the robot, check if it can move.
It can move if:
* it is moving into an empty space or,
* if it is pushing into a box, and that box can move
Boxes have the same rule as the robot for movement.
So a little recursion, moving the boxes and robot as we pop out.

## Part 2

Double wide boxes.
Up and down become an issue, and my single recursion check doesn't work because of the possibility of offsetting boxes.

I originally thought it would just be adding an `&&` and adding a check, but it breaks because we need a failure if the whole recursive tree fails. The way I did it originally had cases where one box or a part of a box would go ahead and move before other branches failed.

So I just split up the function into two parts:
`can_move_object` and `move_object`.
Lots of copy/paste, so not the cleanest code, but refactoring wouldn't be too tough.


## Other notes



## ChatGPT

