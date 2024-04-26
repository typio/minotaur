use ::rand::seq::IteratorRandom;
use ::rand::thread_rng;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
enum CellType {
    Wall,
    Hall,
}

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Dir::Up => write!(f, "Up"),
            Dir::Down => write!(f, "Down"),
            Dir::Left => write!(f, "Left"),
            Dir::Right => write!(f, "Right"),
        }
    }
}

struct Coord {
    x: usize,
    y: usize,
}

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 10;

    let maze_size = Coord { x: w / s, y: h / s };

    let mut maze = vec![CellType::Wall; (maze_size.x * maze_size.y) as usize];

    let mut image = Image::gen_image_color(maze_size.x as u16, maze_size.y as u16, WHITE);

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    // DFS WIP
    let mut walker = Coord {
        x: 0,
        y: maze_size.y / 2,
    };

    loop {
        // Update
        maze[walker.x + walker.y * maze_size.x] = CellType::Hall;

        let mut dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
        while dirs.len() > 0 {
            let dir_i = (0..dirs.len()).choose(&mut thread_rng()).unwrap();
            let dir = dirs.swap_remove(dir_i);

            // out of bounds
            if (walker.x == 0 && dir == Dir::Left)
                || (walker.x == maze_size.x - 1 && dir == Dir::Right)
                || (walker.y == 0 && dir == Dir::Up)
                || (walker.y == maze_size.y - 1 && dir == Dir::Down)
            {
                continue;
            }

            match dir {
                Dir::Up => walker.y -= 1,
                Dir::Down => walker.y += 1,
                Dir::Right => walker.x += 1,
                Dir::Left => walker.x -= 1,
            }

            break;
        }

        // Draw
        clear_background(WHITE);

        let maze_w = image.width();

        for i in 0..maze.len() {
            image.set_pixel(
                (i % maze_w) as u32,
                (i / maze_w) as u32,
                match maze[i as usize] {
                    CellType::Wall => BLACK,
                    CellType::Hall => WHITE,
                },
            );
        }

        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: screen_width() as f32,
                    y: screen_height() as f32,
                }),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        next_frame().await
    }
}
