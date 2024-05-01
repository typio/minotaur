use macroquad::prelude::*;

use crate::{maze_generator::Maze, CellType};

pub struct Renderer {
    image: Image,
    texture: Texture2D,
}

impl Renderer {
    pub fn new(maze: &Maze) -> Self {
        let image = Image::gen_image_color(maze.size.x as u16, maze.size.y as u16, WHITE);
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        Renderer { image, texture }
    }

    pub fn draw(&mut self, maze: &Maze) {
        clear_background(WHITE);

        let image_w = self.image.width();

        for i in 0..maze.map.len() {
            self.image.set_pixel(
                (i % image_w) as u32,
                (i / image_w) as u32,
                if maze.start.flatten(maze.size) == i {
                    GREEN
                } else if maze.end.flatten(maze.size) == i {
                    RED
                } else {
                    match maze.map[i as usize] {
                        CellType::Wall => BLACK,
                        CellType::Tunnel => GRAY,
                        CellType::Hall => WHITE,
                    }
                },
            );
        }

        self.texture.update(&self.image);

        draw_texture_ex(
            &self.texture,
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
    }

    pub fn wait_fps(&self, minimum_frame_time: f32) {
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
    }
}
