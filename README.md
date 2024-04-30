# Minotaur

Yet another maze generator in rust. My main goal is to relearn Rust for a more serious project, 
to that end I'm going to implement a few different algorithms for creating a maze and for solving one.

## Generation
#### Randomized Depth-First Search
<img src="https://github.com/typio/minotaur/assets/26017543/c57b62db-73e3-41c2-bfd9-9b8cd1ece090" width="360"  />
</br></br>
A simple algorithm, which works by moving in a random direction at each step. If there are no valid adjacent directions, it marks the current cell as finished (in white) and backtracks to a neighboring unfinished gray cell. The end of the maze is chosen as the maximum depth reached, and is shown in red. Checks are in place to ensure that the cell can't break a wall that would join two corridors.
