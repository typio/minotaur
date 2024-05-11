use crate::{maze::MazeState, solver::Solver, Dir, Point};

#[derive(Clone, Copy)]
struct Node {
    pos: Point,
    parent: Option<usize>,
}

pub struct BFS {
    finished: bool,
    found_end: bool,
    nodes: Vec<Node>,
    queue: Vec<usize>,
    solution_walker: usize,
}

impl Solver for BFS {
    fn new(maze: &mut crate::maze::Maze) -> Self {
        maze.visited[maze.start.flatten(maze.size)] = 1;

        BFS {
            finished: false,
            found_end: false,
            nodes: vec![Node {
                pos: maze.start,
                parent: None,
            }],
            queue: vec![0],
            solution_walker: 0,
        }
    }

    fn step(&mut self, maze: &mut crate::maze::Maze, speed: usize) {
        for _ in 0..=speed {
            if self.queue.len() > 0 {
                if !self.found_end {
                    let current_node_index = self.queue.pop().unwrap();
                    let current_node = self.nodes[current_node_index];
                    if current_node.pos == maze.end {
                        self.nodes.push(Node {
                            pos: maze.end,
                            parent: Some(current_node_index),
                        });
                        self.solution_walker = self.nodes.len() - 1;
                        self.found_end = true;
                    } else {
                        let current_cell = maze.map[current_node.pos.flatten(maze.size)];

                        for dir in Dir::get_iter() {
                            if !current_cell.walls[dir] {
                                let (dx, dy) = dir.get_xy();
                                let neighbor = Point {
                                    x: (current_node.pos.x as isize + dx) as usize,
                                    y: (current_node.pos.y as isize + dy) as usize,
                                };

                                if maze.visited[neighbor.flatten(maze.size)] == 0 {
                                    maze.visited[neighbor.flatten(maze.size)] = self.queue.len();
                                    self.nodes.push(Node {
                                        pos: neighbor,
                                        parent: Some(current_node_index),
                                    });
                                    self.queue.insert(0, self.nodes.len() - 1);
                                }
                            }
                        }
                    }
                }
            }

            if self.found_end {
                if let Some(walker_parent) = self.nodes[self.solution_walker].parent {
                    maze.solution.push(self.nodes[self.solution_walker].pos);
                    self.solution_walker = walker_parent;
                } else {
                    maze.solution.push(self.nodes[self.solution_walker].pos);
                    if *maze.solution.last().unwrap() != maze.start {
                        maze.solution.push(maze.start);
                    }
                    maze.visited = vec![0; maze.visited.len()];
                    self.finished = true;
                }
            }

            if self.finished {
                maze.state = MazeState::Solved;
            }
        }
    }
}
