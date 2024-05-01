use macroquad::prelude::*;

use minotaur::{
    maze_generator::{Maze, MazeGenerator, DFS},
    renderer::Renderer,
    CellType, Coord,
};

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 10;

    let maze_size = Coord { x: w / s, y: h / s };

    let mut maze_generator = DFS::new(maze_size);

    let mut maze: Maze;
    maze = Maze {
        size: maze_size,
        map: vec![CellType::Wall; maze_size.x * maze_size.y],
        start: maze_generator.walker,
        end: maze_generator.walker,

        play: false,
    };

    let mut renderer = Renderer::new(&maze);

    loop {
        if get_keys_pressed().contains(&KeyCode::P) {
            maze.play = !maze.play;
        }

        if get_keys_pressed().contains(&KeyCode::R) {
            maze_generator = DFS::new(maze_size);

            maze = Maze {
                size: maze_size,
                map: vec![CellType::Wall; maze_size.x * maze_size.y],
                start: maze_generator.walker,
                end: maze_generator.walker,

                play: false,
            };
        }

        if maze.play {
            maze_generator.step(&mut maze, 4);
        }

        renderer.draw(&maze);
        renderer.wait_fps(1. / 300.);

        next_frame().await
    }
}
