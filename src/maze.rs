use crate::{Cell, Point};

#[derive(PartialEq)]
pub enum MazeState {
    Empty,
    Generating,
    Generated,
    Solving,
    Solved,
}

pub struct Maze {
    pub state: MazeState,
    pub generating_speed: usize,
    pub solving_speed: usize,
    pub size: Point,
    pub map: Vec<Cell>,
    pub visited: Vec<usize>,
    pub walker: Point,
    pub start: Point,
    pub end: Point,
    pub solution: Vec<Point>,

    pub play: bool,
}

impl Maze {
    pub fn new(maze_size: Point, generating_speed: usize, solving_speed: usize) -> Self {
        let start = Point { x: 0, y: 0 };
        let end = Point {
            x: maze_size.x - 1,
            y: maze_size.y - 1,
        };

        Maze {
            state: MazeState::Empty,
            generating_speed,
            solving_speed,
            size: maze_size,
            map: vec![Cell::new(true); maze_size.x * maze_size.y],
            visited: vec![0; maze_size.x * maze_size.y],
            walker: start,
            start,
            end,
            solution: vec![],
            play: true,
        }
    }

    pub fn reset(&mut self) {
        self.map = vec![Cell::new(true); self.size.x * self.size.y];
        self.visited = vec![0; self.visited.len()];
        self.state = MazeState::Empty;
        self.solution = vec![];
        self.walker = self.start;
    }
}
