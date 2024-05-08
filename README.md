# Minotaur

Yet another maze generator in rust. My main goal is to relearn Rust for a more serious project, 
to that end I'm going to implement a few different algorithms for creating a maze and for solving one.

## Generation
### Depth-First Search
<img src="https://github.com/typio/minotaur/assets/26017543/0b071f20-d054-4b03-8979-09d5f6a92a5a" width="720"  />
</br></br>
A simple algorithm, which works by adding an adjacent unvisited cell to a stack, then removing the wall between the current cell and the new cell, adding the new cell to the stack, and marking it as visited. It the pops the top cell from the stack and repeats, ending when the stack is empty. The deepest part of the maze reached is chosen as the end, it is the cell marked in red. This is also known as Recursive Backtracking, but I substituted recursion for an explicit stack.
</br></br>

### Kruskal's Algorithm
<img src="https://github.com/typio/minotaur/assets/26017543/5d3ba354-a6a9-4aee-877e-e2be4fa53fc8" width="720"  />
</br></br>
This algorithm works by initializing every cell in a unique (mathmatical) set, then on every iteration it picks a random wall in the maze and if the cells on either side of it are in different set, it merges the sets and removes the wall. Essentially this algorithm removes walls with a random sampling until each cell is connected to any other cell by some path.
