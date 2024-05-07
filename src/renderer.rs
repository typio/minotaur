use crate::maze_generator::Maze;
use macroquad::prelude::*;

pub struct Renderer {
    stroke_width: f32,
    padding: Vec2,
    fill_gap: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            stroke_width: 2.0,
            padding: Vec2 { x: 40., y: 40. },
            fill_gap: 5.,
        }
    }
    pub fn draw(&mut self, maze: &Maze) {
        clear_background(BLACK);

        let screen_size = vec2(screen_width(), screen_height());
        let maze_size = vec2(maze.size.x as f32, maze.size.y as f32);
        let cell_size =
            (screen_size - self.padding * 2.0 - vec2(self.stroke_width, self.stroke_width))
                / (maze_size);

        if maze.play {
            for i in 0..maze.visited.len() {
                if !maze.visited[i] {
                    continue;
                }

                let pos = vec2(
                    (i % maze.size.x) as f32 * cell_size.x + self.padding.x,
                    (i / maze.size.x) as f32 * cell_size.y + self.padding.y,
                );
                draw_rectangle(
                    pos.x - self.stroke_width / 2.,
                    pos.y - self.stroke_width / 2.,
                    cell_size.x + self.stroke_width,
                    cell_size.y + self.stroke_width,
                    DARKGRAY,
                );
            }
        }

        if maze.play {
            let walker_pos =
                vec2(maze.walker.x as f32, maze.walker.y as f32) * (cell_size) + self.padding;
            draw_rectangle(
                walker_pos.x - self.stroke_width / 2.,
                walker_pos.y - self.stroke_width / 2.,
                cell_size.x + self.stroke_width,
                cell_size.y + self.stroke_width,
                BLUE,
            );
        }

        let end_pos = vec2(maze.end.x as f32, maze.end.y as f32) * (cell_size) + self.padding;
        draw_rectangle(
            end_pos.x - self.stroke_width / 2.,
            end_pos.y - self.stroke_width / 2.,
            cell_size.x + self.stroke_width,
            cell_size.y + self.stroke_width,
            RED,
        );

        let start_pos = vec2(maze.start.x as f32, maze.start.y as f32) * (cell_size) + self.padding;
        draw_rectangle(
            start_pos.x - self.stroke_width / 2.,
            start_pos.y - self.stroke_width / 2.,
            cell_size.x + self.stroke_width,
            cell_size.y + self.stroke_width,
            GREEN,
        );

        let line_length = cell_size + self.stroke_width;
        for i in 0..maze.map.len() {
            let pos = vec2(
                (i % maze.size.x) as f32 * cell_size.x + self.padding.x,
                (i / maze.size.x) as f32 * cell_size.y + self.padding.y,
            );

            let cell = maze.map[i];

            self.draw_wall_line(true, pos, line_length, cell.up, 1., 0.);
            self.draw_wall_line(true, pos, line_length, cell.left, 0., 1.);
            self.draw_wall_line(
                true,
                pos + vec2(cell_size.x, 0.),
                line_length,
                cell.right,
                0.,
                1.,
            );
            self.draw_wall_line(
                true,
                pos + vec2(0., cell_size.y),
                line_length,
                cell.down,
                1.,
                0.,
            );
        }

        for i in 0..maze.map.len() {
            let pos = vec2(
                (i % maze.size.x) as f32 * cell_size.x + self.padding.x,
                (i / maze.size.x) as f32 * cell_size.y + self.padding.y,
            );

            let cell = maze.map[i];

            self.draw_wall_line(false, pos, line_length, cell.up, 1., 0.);
            self.draw_wall_line(false, pos, line_length, cell.left, 0., 1.);
            self.draw_wall_line(
                false,
                pos + vec2(cell_size.x, 0.),
                line_length,
                cell.right,
                0.,
                1.,
            );
            self.draw_wall_line(
                false,
                pos + vec2(0., cell_size.y),
                line_length,
                cell.down,
                1.,
                0.,
            );
        }
    }

    fn draw_wall_line(
        &self,
        is_background: bool,
        pos: Vec2,
        line_length: Vec2,
        should_draw: bool,
        dx: f32,
        dy: f32,
    ) {
        if should_draw {
            if is_background {
                let start = pos
                    - vec2(
                        (self.fill_gap + self.stroke_width / 2.) * dx,
                        (self.fill_gap + self.stroke_width / 2.) * dy,
                    );
                let end = start
                    + vec2(
                        (line_length.x + self.fill_gap + self.stroke_width * 2.) * dx,
                        (line_length.y + self.fill_gap + self.stroke_width * 2.) * dy,
                    );
                draw_line(
                    start.x,
                    start.y,
                    end.x,
                    end.y,
                    self.stroke_width + self.fill_gap * 2.,
                    BLACK,
                );
            } else {
                let start = pos - vec2(self.stroke_width / 2. * dx, self.stroke_width / 2. * dy);
                let end = start + vec2(line_length.x * dx, line_length.y * dy);
                draw_line(start.x, start.y, end.x, end.y, self.stroke_width, WHITE);
            }
        }
    }

    pub fn wait_fps(&self, minimum_frame_time: f32) {
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
    }
}

