use crate::disjoint_set::DisjointSet;
use crate::generator::{Generator, Maze};
use crate::Coord;
use macroquad::rand::ChooseRandom;

enum WallDir {
    Down,
    Right,
}

struct Wall {
    pos: Coord,
    dir: WallDir,
}

pub struct Kruskal {
    disjoint_set: DisjointSet,
    walls: Vec<Wall>,
}

impl Generator for Kruskal {
    fn new(maze: &mut Maze) -> Self {
        maze.end = Coord {
            x: maze.size.x - 1,
            y: maze.size.y / 2,
        };

        let mut walls = Vec::new();
        for y in 0..maze.size.y {
            for x in 0..maze.size.x {
                let pos = Coord { x, y };
                if x < maze.size.x - 1 {
                    walls.push(Wall {
                        pos,
                        dir: WallDir::Right,
                    });
                }
                if y < maze.size.y - 1 {
                    walls.push(Wall {
                        pos,
                        dir: WallDir::Down,
                    });
                }
            }
        }
        walls.shuffle();

        let mut disjoint_set = DisjointSet::new();
        for i in 0..maze.size.x * maze.size.y {
            disjoint_set.make_set(i);
        }

        Kruskal {
            disjoint_set,
            walls,
        }
    }

    fn step(&mut self, maze: &mut Maze, speed: usize) {
        for _ in 0..=speed {
            if let Some(wall) = self.walls.pop() {
                let (c1, c2) = match wall.dir {
                    WallDir::Down => (
                        wall.pos.flatten(maze.size),
                        Coord {
                            x: wall.pos.x,
                            y: wall.pos.y + 1,
                        }
                        .flatten(maze.size),
                    ),
                    WallDir::Right => (
                        wall.pos.flatten(maze.size),
                        Coord {
                            x: wall.pos.x + 1,
                            y: wall.pos.y,
                        }
                        .flatten(maze.size),
                    ),
                };

                if self.disjoint_set.find_set(c1) != self.disjoint_set.find_set(c2) {
                    self.disjoint_set.union(c1, c2);
                    match wall.dir {
                        WallDir::Down => {
                            maze.map[wall.pos.flatten(maze.size)].down = false;
                            maze.map[Coord {
                                x: wall.pos.x,
                                y: wall.pos.y + 1,
                            }
                            .flatten(maze.size)]
                            .up = false;
                        }
                        WallDir::Right => {
                            maze.map[wall.pos.flatten(maze.size)].right = false;
                            maze.map[Coord {
                                x: wall.pos.x + 1,
                                y: wall.pos.y,
                            }
                            .flatten(maze.size)]
                            .left = false;
                        }
                    }
                }
            } else {
                maze.play = false;
                break;
            }
        }
    }
}

