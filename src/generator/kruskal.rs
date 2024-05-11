use crate::disjoint_set::DisjointSet;
use crate::generator::Generator;
use crate::maze::{Maze, MazeState};
use crate::Point;
use macroquad::rand::ChooseRandom;

enum WallDir {
    Down,
    Right,
}

struct Wall {
    pos: Point,
    dir: WallDir,
}

pub struct Kruskal {
    disjoint_set: DisjointSet,
    walls: Vec<Wall>,
}

impl Generator for Kruskal {
    fn new(maze: &mut Maze) -> Self {
        let mut walls = Vec::new();
        for y in 0..maze.size.y {
            for x in 0..maze.size.x {
                let pos = Point { x, y };
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
                        Point {
                            x: wall.pos.x,
                            y: wall.pos.y + 1,
                        }
                        .flatten(maze.size),
                    ),
                    WallDir::Right => (
                        wall.pos.flatten(maze.size),
                        Point {
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
                            maze.map[wall.pos.flatten(maze.size)].walls.down = false;
                            maze.map[Point {
                                x: wall.pos.x,
                                y: wall.pos.y + 1,
                            }
                            .flatten(maze.size)]
                            .walls
                            .up = false;
                        }
                        WallDir::Right => {
                            maze.map[wall.pos.flatten(maze.size)].walls.right = false;
                            maze.map[Point {
                                x: wall.pos.x + 1,
                                y: wall.pos.y,
                            }
                            .flatten(maze.size)]
                            .walls
                            .left = false;
                        }
                    }
                }
            } else {
                maze.state = MazeState::Generated;
                // maze.play = false;
                break;
            }
        }
    }
}

