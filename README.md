sokoban
=======

A simple sokoban solver written in Rust 0.7. Its main goal is to find a non-optimal solution with average speed (< 1min).
I am doing it mainly to learn both problem solving and how to code in Rust by myself.
So if you are knowledgeable in one of these fields, I'm open to any suggestion to improve this code.

## How to use it

Just launch on a terminal the binary with the file path as an argument. example : ` ./sokoban test.xsb `
The solver will accept any file formatted in `.xsb` as described [there](http://sokosolve.sourceforge.net/FileFormatXSB.html) :

* `' '` for a floor tile
* `'#'` for a wall
* `'.'` for a target
* `'$'` for a box
* `'*'` for a box on a target
* `'@'` represents the player
* `'+'` represents the player on a target

The solver will then print each pushes that goes from the initial position to the solution.

## How does it works

The algorithm is basically a bruteforce that recursively computes each possible positions.
It is backed by a container that keeps track of what have alreay been done to avoid doing a position twice. 
In addition, deadlocks (tile where a box is stuck as in a corner) are computed once from the initial position and then cutted down from the possible paths.

For each passes the algorithm computes (that's simplified) : 
* A mask of all possible player moves without pushing any box.
* A mask of all possible box pushes, whatever the player position is.
* The intersect of these two masks, giving the true possible pushes.

All data are stored in Bitv structures, which are basically bits stored in an array of uint, that make operations quite cheap. 
There is also added support for `>>` and `<<` that enable shifting a position to the up/left/right/down. 
For example :
* `boxes << 1` would represent all tiles at the right of a box.
* `floor & (boxes << 1)` would be used to know if there's room to push the boxes to the left.
* `player & (boxes >> 1)` would be used to know if the player can be at the right of the boxes to push.
There, you know which boxes can be pushed left.

## What does not work

* The solver will output all pushes except the last one... Hopefully, I think anyone would find this one.
* The output shows box on a target as as box and player on a target as a player.
* The solver accepts files that have 0 or more than 1 player on it.

## What can be made better

* There's a lot of Bitv cloning as each operation allocates a new Bitv.
* The algo might be done with less Bitv operations.
* All deadlocks are not computed. Only corner ones are for now.
* The solver is not parallelized at all for now.
* Upgrading the Rust compiler to 0.8 when it lands could be a win.
