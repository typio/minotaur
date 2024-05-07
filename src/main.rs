use macroquad::prelude::*;

use minotaur::{
    maze_generator::{Maze, MazeGenerator, DFS},
    renderer::Renderer,
    Cell, Coord,
};

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 25;

    let maze_size = Coord { x: w / s, y: h / s };
    let start = Coord {
        x: 0,
        y: maze_size.y / 2,
    };

    let mut maze: Maze;
    maze = Maze {
        size: maze_size,
        map: vec![Cell::closed(); maze_size.x * maze_size.y],
        visited: vec![false; maze_size.x * maze_size.y],
        walker: start,
        start,
        end: start,
        play: false,
    };

    let mut maze_generator = DFS::new(&maze);

    let mut renderer = Renderer::new();

    loop {
        if get_keys_pressed().contains(&KeyCode::P) {
            maze.play = !maze.play;
        }

        if get_keys_pressed().contains(&KeyCode::R) {
            maze_generator = DFS::new(&maze);

            maze = Maze {
                size: maze_size,
                map: vec![Cell::closed(); maze_size.x * maze_size.y],
                visited: vec![false; maze_size.x * maze_size.y],
                walker: start,
                start,
                end: start,
                play: false,
            };
        }

        if maze.play {
            maze_generator.step(&mut maze, 1);
        }

        renderer.draw(&maze);
        renderer.wait_fps(1. / 300.);

        next_frame().await
    }
}
