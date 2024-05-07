# Minotaur

Yet another maze generator in rust. My main goal is to relearn Rust for a more serious project, 
to that end I'm going to implement a few different algorithms for creating a maze and for solving one.

## Generation
#### Randomized Depth-First Search
<img src="https://github.com/typio/minotaur/assets/26017543/5777a488-f52e-4658-8a69-74e6a6e3b407" width="720"  />
</br></br>
A simple algorithm, which works by adding an adjacent unvisited cell to a stack, then removing the wall between the current cell and the new cell, adding the new cell to the stack, and marking it as visited. It the pops the top cell from the stack and repeats, ending when the stack is empty. The deepest part of the maze reached is chosen as the end, it is the cell marked in red.
