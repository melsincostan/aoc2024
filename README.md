# Advent of Code

## Notes and To-Dos

### Day 08

#### Part 2

First instinct here was to compute a function of type f(x) = a*x + b, as compared to just making a vector between two emitters and using factors of this vector to get points.
This was intended to help in the case that there is a smaller possible vector or something ((6, 8) -> (3, 2)) or something. Implementation worked on the sample grid, not on the main solution (didn't get all the points?).
Implementing with factors (start at 1, increment the factor by 1 until a point lands outside the grid) seems to not have that expected issue anyways and works...
