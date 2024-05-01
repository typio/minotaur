use crate::{CellType, Coord, Dir, DIRS};
use rand::{thread_rng, Rng};

pub trait MazeGenerator {
    fn new(maze_size: Coord) -> Self;
    fn step(&mut self, maze: &mut Maze, speed: usize);
    fn is_finished(&self) -> bool;
}

pub struct Maze {
    pub size: Coord,
    pub map: Vec<CellType>,
    pub start: Coord,
    pub end: Coord,

    pub play: bool,
}

pub struct DFS {
    pub walker: Coord,
    pub depth: usize,
    pub max_depth: usize,
}

impl MazeGenerator for DFS {
    fn new(maze_size: Coord) -> Self {
        DFS {
            walker: Coord {
                x: 0,
                y: maze_size.y / 2,
            },
            depth: 0,
            max_depth: 1,
        }
    }

    fn step(&mut self, maze: &mut Maze, speed: usize) {
        for _ in 1..=speed {
            let last_walker = self.walker;

            if self.depth > self.max_depth {
                maze.end = self.walker;
                self.max_depth = self.depth;
            }
            maze.map[self.walker.flatten(maze.size)] = CellType::Tunnel;

            let mut dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
            'walk: loop {
                // nowhere to go so backtrack
                if dirs.len() == 0 {
                    let mut dirs_2 = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
                    while dirs_2.len() > 0 {
                        let dir_2 = dirs_2.swap_remove(0);
                        if !self.walker.is_valid(maze.size, dir_2) {
                            continue;
                        }
                        let prospective_tunnel = match dir_2 {
                            Dir::Up => Coord {
                                x: self.walker.x,
                                y: self.walker.y - 1,
                            },
                            Dir::Down => Coord {
                                x: self.walker.x,
                                y: self.walker.y + 1,
                            },
                            Dir::Right => Coord {
                                x: self.walker.x + 1,
                                y: self.walker.y,
                            },
                            Dir::Left => Coord {
                                x: self.walker.x - 1,
                                y: self.walker.y,
                            },
                        };
                        if maze.map[prospective_tunnel.flatten(maze.size)] == CellType::Tunnel {
                            maze.map[self.walker.x + self.walker.y * maze.size.x] = CellType::Hall;
                            self.walker = prospective_tunnel;
                            self.depth -= 1;
                        }
                    }

                    break;
                }

                let dir = dirs.swap_remove(thread_rng().gen_range(0..dirs.len()));

                // out of bounds
                if !self.walker.is_valid(maze.size, dir) {
                    continue;
                }

                let new_square = match dir {
                    Dir::Up => Coord {
                        x: self.walker.x,
                        y: self.walker.y - 1,
                    },
                    Dir::Down => Coord {
                        x: self.walker.x,
                        y: self.walker.y + 1,
                    },
                    Dir::Right => Coord {
                        x: self.walker.x + 1,
                        y: self.walker.y,
                    },
                    Dir::Left => Coord {
                        x: self.walker.x - 1,
                        y: self.walker.y,
                    },
                };

                // is hall
                match maze.map[new_square.x + new_square.y * maze.size.x] {
                    CellType::Tunnel | CellType::Hall => {
                        continue;
                    }
                    _ => {}
                }

                // breaks single layer wall
                for d in DIRS {
                    // if behind skip check
                    if (dir.get_xy().0 == 1 && d.0 == -1)
                        || (dir.get_xy().0 == -1 && d.0 == 1)
                        || (dir.get_xy().1 == 1 && d.1 == -1)
                        || (dir.get_xy().1 == -1 && d.1 == 1)
                    {
                        continue;
                    } else if (new_square.x == 0 && d.0 == -1)
                        || (new_square.x == maze.size.x - 1 && d.0 == 1)
                        || (new_square.y == 0 && d.1 == -1)
                        || (new_square.y == maze.size.y - 1 && d.1 == 1)
                    {
                        continue;
                    } else if match maze.map[(new_square.x as isize + d.0) as usize
                        + ((new_square.y as isize + d.1) as usize * maze.size.x)]
                    {
                        CellType::Tunnel | CellType::Hall => true,
                        _ => false,
                    } {
                        continue 'walk;
                    }
                }

                self.walker = new_square;
                self.depth += 1;

                break;
            }

            if last_walker.x == self.walker.x && last_walker.y == self.walker.y {
                maze.play = false;
            }
        }
    }

    fn is_finished(&self) -> bool {
        true
    }
}
