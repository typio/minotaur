use macroquad::prelude::*;

use minotaur::{
    generator::{dfs::DFS, kruskal::Kruskal, Generator, Maze},
    renderer::Renderer,
    Coord,
};

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 25;

    let maze_size = Coord { x: w / s, y: h / s };

    let mut maze = Maze::new(maze_size);

    let mut generator = Kruskal::new(&mut maze);

    let mut renderer = Renderer::new();

    loop {
        if get_keys_pressed().contains(&KeyCode::P) {
            maze.play = !maze.play;
        }

        if get_keys_pressed().contains(&KeyCode::R) {
            maze = Maze::new(maze_size);
            generator = Kruskal::new(&mut maze);
        }

        if maze.play {
            generator.step(&mut maze, 4);
        }

        renderer.draw(&maze);
        renderer.wait_fps(1. / 300.);

        next_frame().await
    }
}
