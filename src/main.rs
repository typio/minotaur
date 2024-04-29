use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;

#[derive(Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellType {
    Wall,
    Tunnel,
    Hall,
}

const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (0, 1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn get_xy(&self) -> (isize, isize) {
        match self {
            Dir::Up => DIRS[0],
            Dir::Down => DIRS[1],
            Dir::Right => DIRS[2],
            Dir::Left => DIRS[3],
        }
    }
}

#[macroquad::main("Minotaur")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let s = 10;

    let maze_size = Coord { x: w / s, y: h / s };

    let mut maze = vec![CellType::Wall; maze_size.x * maze_size.y];

    let mut image = Image::gen_image_color(maze_size.x as u16, maze_size.y as u16, WHITE);

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut walker = Coord {
        x: 0,
        y: maze_size.y / 2,
    };

    let mut depth = 0;
    let mut max_depth = 1;
    let start = walker.x + walker.y * maze_size.x;
    let mut end = walker.x + walker.y * maze_size.x;

    let minimum_frame_time = 1. / 300.;

    let mut play = false;

    loop {
        if get_keys_down().contains(&KeyCode::P) {
            play = true;
        }

        // Update
        if play {
            if depth > max_depth {
                end = walker.x + walker.y * maze_size.x;
                max_depth = depth;
            }
            maze[walker.x + walker.y * maze_size.x] = CellType::Tunnel;

            let mut dirs = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
            'walk: loop {
                if dirs.len() == 0 {
                    let mut dirs_2 = vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right];
                    while dirs_2.len() > 0 {
                        let dir_2 = dirs_2.swap_remove(thread_rng().gen_range(0..dirs_2.len()));
                        if (walker.x == 0 && dir_2 == Dir::Left)
                            || (walker.x == maze_size.x - 1 && dir_2 == Dir::Right)
                            || (walker.y == 0 && dir_2 == Dir::Up)
                            || (walker.y == maze_size.y - 1 && dir_2 == Dir::Down)
                        {
                            continue;
                        }
                        let prospective_tunnel = match dir_2 {
                            Dir::Up => Coord {
                                x: walker.x,
                                y: walker.y - 1,
                            },
                            Dir::Down => Coord {
                                x: walker.x,
                                y: walker.y + 1,
                            },
                            Dir::Right => Coord {
                                x: walker.x + 1,
                                y: walker.y,
                            },
                            Dir::Left => Coord {
                                x: walker.x - 1,
                                y: walker.y,
                            },
                        };
                        if maze[prospective_tunnel.x + prospective_tunnel.y * maze_size.x]
                            == CellType::Tunnel
                        {
                            maze[walker.x + walker.y * maze_size.x] = CellType::Hall;
                            walker = prospective_tunnel;
                            depth -= 1;
                        }
                    }

                    break;
                }

                let dir = dirs.swap_remove(thread_rng().gen_range(0..dirs.len()));

                // out of bounds
                if (walker.x == 0 && dir == Dir::Left)
                    || (walker.x == maze_size.x - 1 && dir == Dir::Right)
                    || (walker.y == 0 && dir == Dir::Up)
                    || (walker.y == maze_size.y - 1 && dir == Dir::Down)
                {
                    continue;
                }

                let new_square = match dir {
                    Dir::Up => Coord {
                        x: walker.x,
                        y: walker.y - 1,
                    },
                    Dir::Down => Coord {
                        x: walker.x,
                        y: walker.y + 1,
                    },
                    Dir::Right => Coord {
                        x: walker.x + 1,
                        y: walker.y,
                    },
                    Dir::Left => Coord {
                        x: walker.x - 1,
                        y: walker.y,
                    },
                };

                // is hall
                match maze[new_square.x + new_square.y * maze_size.x] {
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
                        || (new_square.x == maze_size.x - 1 && d.0 == 1)
                        || (new_square.y == 0 && d.1 == -1)
                        || (new_square.y == maze_size.y - 1 && d.1 == 1)
                    {
                        continue;
                    } else if match maze[(new_square.x as isize + d.0) as usize
                        + ((new_square.y as isize + d.1) as usize * maze_size.x)]
                    {
                        CellType::Tunnel | CellType::Hall => true,
                        _ => false,
                    } {
                        continue 'walk;
                    }
                }

                walker = new_square;
                depth += 1;

                break;
            }
        }

        // Draw
        clear_background(WHITE);

        let maze_w = image.width();

        for i in 0..maze.len() {
            image.set_pixel(
                (i % maze_w) as u32,
                (i / maze_w) as u32,
                if start == i {
                    GREEN
                } else if end == i {
                    RED
                } else {
                    match maze[i as usize] {
                        CellType::Wall => BLACK,
                        CellType::Tunnel => GRAY,
                        CellType::Hall => WHITE,
                    }
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

        let frame_time = get_frame_time();
        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

        next_frame().await
    }
}
