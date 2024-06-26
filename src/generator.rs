use crate::maze::Maze;

pub mod dfs;
pub mod kruskal;

pub trait Generator {
    fn new(maze: &mut Maze) -> Self;
    fn step(&mut self, maze: &mut Maze, speed: usize);
}
