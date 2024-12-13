# Day 12: Garden Groups

## Intro

Fill algorithms

## Part 1

For each contiguous block, multiply the perimter by the area, and sum.
Simple enough:
* Pick a pixel
* Run a fill algorithm from that pixel
* For each pixel, check the neighbor in each direction.
* If the neighbor is different, there is an edge there.
* Mark that pixel as visited.
Count the pixels and the edges for that filled area, and multiply.
Repeat for all pixels until they're all visited.

## Part 2

Find the number of edges for each block, rather than the perimeter.
There's probably a more optimal way to do this, but this works and is reasonably quick.
* Use a fill algorithm on a pixel to identify a contiguous block
* Do four passes on that block, one for each possible edge: top, bottom, left, right
* Example nested loop to count top edges:
  * Loop from minimum y to maximum y of the block
    * Loop from minimum x to maximum x of the block
      * Check if the pixel has an edge on the top. If so increment the edge counter
      * Increment x until there is not an edge above, or until the end of the line
* Repeat for bottom, left, and right (adjusting the loop and conditions appropriately)

## Other notes



## ChatGPT

