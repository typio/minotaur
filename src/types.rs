use std::ops::Index;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn is_valid(&self, maze_size: Point, dir: Dir) -> bool {
        if maze_size.x < 1 || maze_size.y < 1 {
            panic!("Maze size must be greater than 1 both x and y.");
        }
        !((self.x == 0 && dir == Dir::Left)
            || (self.x == maze_size.x - 1 && dir == Dir::Right)
            || (self.y == 0 && dir == Dir::Up)
            || (self.y == maze_size.y - 1 && dir == Dir::Down))
    }
    pub fn flatten(&self, maze_size: Point) -> usize {
        self.x + self.y * maze_size.x
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub walls: Walls,
}

impl Cell {
    pub fn new(is_closed: bool) -> Self {
        Cell {
            walls: if is_closed {
                Walls::closed()
            } else {
                Walls::open()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Walls {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Walls {
    pub fn closed() -> Self {
        Walls {
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }
    pub fn open() -> Self {
        Walls {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
impl Index<Dir> for Walls {
    type Output = bool;

    fn index(&self, dir: Dir) -> &Self::Output {
        match dir {
            Dir::Up => &self.up,
            Dir::Down => &self.down,
            Dir::Left => &self.left,
            Dir::Right => &self.right,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter)]
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
    pub fn get_iter() -> DirIter {
        Dir::iter()
    }
}

pub const DIRS: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
