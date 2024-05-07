#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn is_valid(&self, maze_size: Coord, dir: Dir) -> bool {
        if maze_size.x < 1 || maze_size.y < 1 {
            panic!("Maze size must be greater than 1 both x and y.");
        }
        !((self.x == 0 && dir == Dir::Left)
            || (self.x == maze_size.x - 1 && dir == Dir::Right)
            || (self.y == 0 && dir == Dir::Up)
            || (self.y == maze_size.y - 1 && dir == Dir::Down))
    }
    pub fn flatten(&self, maze_size: Coord) -> usize {
        self.x + self.y * maze_size.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Cell {
    pub fn closed() -> Self {
        Cell {
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn get_xy(&self) -> (isize, isize) {
        match self {
            Dir::Up => DIRS[0],
            Dir::Down => DIRS[1],
            Dir::Right => DIRS[2],
            Dir::Left => DIRS[3],
        }
    }
}

pub const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
