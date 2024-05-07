use crate::{Cell, Coord, Dir};
use rand::{thread_rng, Rng};

pub trait MazeGenerator {
    fn new(maze: &Maze) -> Self;
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

pub struct DFS {
    pub depth: usize,
    pub max_depth: usize,
    pub stack: Vec<Coord>,
}

impl MazeGenerator for DFS {
    fn new(maze: &Maze) -> Self {
        DFS {
            depth: 1,
            max_depth: 1,
            stack: vec![maze.start],
        }
    }

    fn step(&mut self, maze: &mut Maze, speed: usize) {
        for _ in 1..=speed {
            if self.stack.len() == 0 {
                break;
            }
            let current_cell = self.stack.pop().expect("Stack can't be empty");
            self.depth -= 1;
            maze.walker = current_cell;

            let mut dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
            loop {
                if dirs.len() == 0 {
                    break;
                }

                let dir = dirs.swap_remove(thread_rng().gen_range(0..dirs.len()));
                if !current_cell.is_valid(maze.size, dir) {
                    continue;
                }
                let next = match dir {
                    Dir::Up => Coord {
                        x: current_cell.x,
                        y: current_cell.y - 1,
                    },
                    Dir::Down => Coord {
                        x: current_cell.x,
                        y: current_cell.y + 1,
                    },
                    Dir::Right => Coord {
                        x: current_cell.x + 1,
                        y: current_cell.y,
                    },
                    Dir::Left => Coord {
                        x: current_cell.x - 1,
                        y: current_cell.y,
                    },
                };

                let next_i = next.flatten(maze.size);

                if !maze.visited[next_i] {
                    self.stack.push(current_cell);
                    self.depth += 1;

                    let current_i = current_cell.flatten(maze.size);
                    match dir {
                        Dir::Up => {
                            maze.map[current_i].up = false;
                            maze.map[next_i].down = false;
                        }
                        Dir::Down => {
                            maze.map[current_i].down = false;
                            maze.map[next_i].up = false;
                        }
                        Dir::Right => {
                            maze.map[current_i].right = false;
                            maze.map[next_i].left = false;
                        }
                        Dir::Left => {
                            maze.map[current_i].left = false;
                            maze.map[next_i].right = false;
                        }
                    }

                    maze.visited[next.flatten(maze.size)] = true;
                    self.stack.push(next);
                    self.depth += 1;
                    break;
                } else {
                    continue;
                }
            }

            if self.depth > self.max_depth {
                self.max_depth = self.depth;
                maze.end = current_cell;
            }

            if self.stack.len() == 0 {
                maze.play = false;
            }
        }
    }
}
