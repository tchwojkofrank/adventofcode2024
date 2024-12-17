# Day 16: Reindeer Maze

## Intro

Shortest path with weighted costs

## Part 1

Hey I have a library ready for this! I had generic implementations for Dijkstra and A* for shortest path work. I just needed to implement for a Node:
* a get_neighbors function
* a cost function to get the weight of an edge
* a heuristic function

Because the map has costs that have penalties for turning, my nodes have both position and direction.

The heuristic function can just be set to always return 0, neither algorithm requires anything more (it just could improve performance a bit).

Neighbors were easy enough: get the neighboring cells that aren't walls, and set the direction to the direction the path would need to take to get there.

The edge weight would be either 1 or 1001 depending if a turn was involved. (A 180 turn is nonsensical as you'd just go back where you came from, obviously making the path longer).

I realized that approach wouldn't work, as some positions would come in from different directions, and wouldn't necessarily catch all the states.

So I changed the neighbor function to return the map position in the same current direction (if it was not a wall), or turning clockwise or counter-clockwise.

Now the cost was either 1 or 1000. (Turning 180 is still not helpful.)

I started with Dijkstra's shortest path, but there seemed to be a bug. I didn't see anything obvious, so I tried my A* implementation. That also had bugs.

I got the A* bug fixed, and it turned out I had similar issues in the Dijkstra solution. I fixed both and they both got the answer right.

## Part 2

Now get all the shortest paths. Dijkstra can do this, A* can't. So I added to the library and adjusted a copy of the get_shortest_path function to return all the paths. This required changing the return value from a `HashMap<Node,Node>` to `HashMap<Node,Vec<Node>>`.

There was a bug in this that took a while to sort out, but eventually got there.

We'll see if I ever have to use this again this year.

## Other notes



## ChatGPT

