pub mod dfs;
pub mod kruskal;

use crate::{Cell, Coord};

pub trait Generator {
    fn new(maze: &mut Maze) -> Self;
    fn step(&mut self, maze: &mut Maze, speed: usize);
}

pub struct Maze {
    pub size: Coord,
    pub map: Vec<Cell>,
    pub visited: Vec<bool>,
    pub walker: Coord,
    pub start: Coord,
    pub end: Coord,

    pub play: bool,
}

impl Maze {
    pub fn new(maze_size: Coord) -> Self {
        let start = Coord {
            x: 0,
            y: maze_size.y / 2,
        };
        Maze {
            size: maze_size,
            map: vec![Cell::closed(); maze_size.x * maze_size.y],
            visited: vec![false; maze_size.x * maze_size.y],
            walker: start,
            start,
            end: start,
            play: false,
        }
    }
}
