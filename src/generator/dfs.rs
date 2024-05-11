use crate::maze::{Maze, MazeState};
use crate::{generator::Generator, Dir, Point};

use rand::{thread_rng, Rng};

pub struct DFS {
    pub stack: Vec<Point>,
}

impl Generator for DFS {
    fn new(maze: &mut Maze) -> Self {
        DFS {
            stack: vec![maze.start],
        }
    }

    fn step(&mut self, maze: &mut Maze, speed: usize) {
        for _ in 1..=speed {
            if self.stack.len() == 0 {
                break;
            }
            let current_cell = self.stack.pop().expect("Stack can't be empty");
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
                    Dir::Up => Point {
                        x: current_cell.x,
                        y: current_cell.y - 1,
                    },
                    Dir::Down => Point {
                        x: current_cell.x,
                        y: current_cell.y + 1,
                    },
                    Dir::Right => Point {
                        x: current_cell.x + 1,
                        y: current_cell.y,
                    },
                    Dir::Left => Point {
                        x: current_cell.x - 1,
                        y: current_cell.y,
                    },
                };

                let next_i = next.flatten(maze.size);

                if maze.visited[next_i] == 0 {
                    self.stack.push(current_cell);

                    let current_i = current_cell.flatten(maze.size);
                    match dir {
                        Dir::Up => {
                            maze.map[current_i].walls.up = false;
                            maze.map[next_i].walls.down = false;
                        }
                        Dir::Down => {
                            maze.map[current_i].walls.down = false;
                            maze.map[next_i].walls.up = false;
                        }
                        Dir::Right => {
                            maze.map[current_i].walls.right = false;
                            maze.map[next_i].walls.left = false;
                        }
                        Dir::Left => {
                            maze.map[current_i].walls.left = false;
                            maze.map[next_i].walls.right = false;
                        }
                    }

                    maze.visited[next.flatten(maze.size)] = 1;
                    self.stack.push(next);
                    break;
                } else {
                    continue;
                }
            }

            if self.stack.len() == 0 {
                maze.state = MazeState::Generated;
                maze.visited = vec![0; maze.visited.len()];
            }
        }
    }
}
