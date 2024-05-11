use macroquad::prelude::*;

use minotaur::{
    generator::{dfs::DFS, kruskal::Kruskal, Generator},
    maze::{Maze, MazeState},
    renderer::Renderer,
    solver::{bfs::BFS, Solver},
    Point,
};

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 18;

    let mut padding = Vec2 { x: 50., y: 50. };
    let available_size = vec2(w as f32 - padding.x * 2.0, h as f32 - padding.y * 2.0);

    let cell_size = (available_size / available_size).min_element().floor();

    let remaining_size = available_size - (available_size * cell_size);

    padding += remaining_size / 2.;

    let mut maze = Maze::new(
        Point {
            x: available_size.x as usize / s,
            y: available_size.y as usize / s,
        },
        20,
        20,
    );

    let mut generator = Kruskal::new(&mut maze);
    let mut solver = BFS::new(&mut maze);

    let mut renderer = Renderer::new(padding).await;

    loop {
        if get_keys_pressed().contains(&KeyCode::P) {
            maze.play = !maze.play;
        }

        if get_keys_pressed().contains(&KeyCode::R) {
            maze.reset();
            generator = Kruskal::new(&mut maze);
            solver = BFS::new(&mut maze);
        }

        if get_keys_pressed().contains(&KeyCode::G) {
            if maze.state == MazeState::Empty {
                maze.state = MazeState::Generating;
            }
        }

        if get_keys_pressed().contains(&KeyCode::S) {
            if maze.state == MazeState::Generated {
                maze.state = MazeState::Solving;
            }
        }

        if maze.play {
            match maze.state {
                MazeState::Empty => {}
                MazeState::Generating => {
                    generator.step(&mut maze, 4000);
                }
                MazeState::Generated => {}
                MazeState::Solving => solver.step(&mut maze, 3),
                MazeState::Solved => {}
            }
        }

        renderer.draw(&maze);
        renderer.wait_fps(1. / 300.);

        next_frame().await
    }
}
