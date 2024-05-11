use crate::maze::Maze;

pub mod bfs;

pub trait Solver {
    fn new(maze: &mut Maze) -> Self;
    fn step(&mut self, maze: &mut Maze, speed: usize);
}
