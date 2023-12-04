# A Repo for the Advent of Code Challenge 2023

I made this repo to keep track of my [Advent of Code 2023](https://adventofcode.com/2023).

Each day is its own folder. Done in Rust as I am trying to learn it I hope I can finish the advent calendar since this is my first try.

**I have included the input files as well, to run the code of any day you can -**
> cd .\day-xx\
> cargo run

**And the output of part 2 will be displayed**

## Day 01 - 
Thought it was a code competition but turns out it's just the answer that is necessary.
Used brute force, first converted all strings so that each word i.e., *"one"*, *"two"* and so on are replaced by their numbers i.e., *"1"*, *"2"* and so on.
After that its a simple matter of iterating over string backwards and forwards, picking up the numbers and adding them

## Day 02 -
A bunch of string splitting, and then just adding and making sure that red, green, and blue balls are all withing their parameters.
Part 2 is just finding max red, blue, and green from each line and multiplying and adding.

## Day 03 - 
Took some time, overflowed into day 04. Had to brute force it in the end, find out what numbers are at each index and create a lookup_table. Proved handy in 
solving part 2 as well.

## Day 04 -
Part 1 was fairly easy, it was just a series of string splits, and then checking if number is in the vector or not. Part 2 was a bit tricky to understand. But in the end implementation was fairly straightforward so it was not too tough