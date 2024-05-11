use crate::{maze::Maze, Dir, Point};
use macroquad::prelude::*;

pub struct Renderer {
    font: Font,
    stroke_width: f32,
    padding: Vec2,
    fill_gap: f32,
    fps_history: Vec<i32>,
}

impl Renderer {
    pub async fn new(padding: Vec2) -> Self {
        let mut font = load_ttf_font("./res/fonts/04B_03B_.ttf").await.unwrap();
        font.set_filter(FilterMode::Nearest);

        Renderer {
            font,
            stroke_width: 4.0,
            padding,
            fill_gap: 3.,
            fps_history: vec![],
        }
    }

    pub fn draw(&mut self, maze: &Maze) {
        clear_background(BLACK);

        let screen_size = vec2(screen_width(), screen_height());

        let available_size = vec2(
            screen_size.x - self.padding.x * 2.0 - self.stroke_width * 2.0,
            screen_size.y - self.padding.y * 2.0 - self.stroke_width * 2.0,
        );

        let maze_size = vec2(maze.size.x as f32, maze.size.y as f32);

        let cell_size = (available_size / maze_size).min_element().floor();

        let remaining_size = available_size - (maze_size * cell_size);

        self.padding += remaining_size / 2.;

        // Draw visited
        for i in 0..maze.visited.len() {
            if maze.visited[i] == 0 {
                continue;
            }

            let pos = Point {
                x: (i % maze.size.x),
                y: (i / maze.size.x),
            };

            self.draw_cell(
                pos,
                cell_size,
                Color {
                    r: 120.,
                    g: 0.,
                    b: 255.,
                    a: 0.2
                        + ((0.7 - 0.2) / (1.)
                            * (maze.visited[i] as f32
                                / *maze.visited.iter().max().unwrap() as f32)),
                },
            );
        }

        // Draw walker
        if maze.play {
            self.draw_cell(maze.walker, cell_size, BLUE);
        }

        // Draw solution
        if maze.solution.len() > 2 {
            let mut prev_coord = maze.solution[0];
            let mut start_coord = maze.solution[0];
            let mut moved_dir: Dir = if maze.solution[1].y < prev_coord.y {
                Dir::Up
            } else if maze.solution[1].y > prev_coord.y {
                Dir::Down
            } else if maze.solution[1].x > prev_coord.x {
                Dir::Right
            } else {
                Dir::Left
            };

            let mut last_moved_dir: Dir = moved_dir;

            for (i, current_coord) in maze.solution[1..].iter().enumerate() {
                moved_dir = if current_coord.y < prev_coord.y {
                    Dir::Up
                } else if current_coord.y > prev_coord.y {
                    Dir::Down
                } else if current_coord.x > prev_coord.x {
                    Dir::Right
                } else {
                    Dir::Left
                };

                if moved_dir != last_moved_dir || i == maze.solution.len() - 2 {
                    let (start_pos, end_pos) = if start_coord.x <= prev_coord.x
                        && start_coord.y <= prev_coord.y
                    {
                        (
                            vec2(start_coord.x as f32, start_coord.y as f32) * cell_size
                                + self.padding
                                + vec2(self.stroke_width / 2., self.stroke_width / 2.),
                            vec2(prev_coord.x as f32 + 1., prev_coord.y as f32 + 1.) * cell_size
                                + self.padding
                                - vec2(self.stroke_width / 2., self.stroke_width / 2.),
                        )
                    } else {
                        (
                            vec2(prev_coord.x as f32, prev_coord.y as f32) * cell_size
                                + self.padding
                                + vec2(self.stroke_width / 2., self.stroke_width / 2.),
                            vec2(start_coord.x as f32 + 1., start_coord.y as f32 + 1.) * cell_size
                                + self.padding
                                - vec2(self.stroke_width / 2., self.stroke_width / 2.),
                        )
                    };

                    let rect_width = match last_moved_dir {
                        Dir::Up | Dir::Down => cell_size - self.stroke_width - self.fill_gap * 2.,
                        _ => (end_pos.x - start_pos.x) - self.fill_gap * 2.,
                    };

                    let rect_height = match last_moved_dir {
                        Dir::Left | Dir::Right => {
                            cell_size - self.stroke_width - self.fill_gap * 2.
                        }
                        _ => (end_pos.y - start_pos.y) - self.fill_gap * 2.,
                    };

                    draw_rectangle(
                        start_pos.x + self.fill_gap,
                        start_pos.y + self.fill_gap,
                        rect_width,
                        rect_height,
                        BLUE,
                    );
                    start_coord = prev_coord;
                }

                prev_coord = *current_coord;
                last_moved_dir = moved_dir;
            }
        }

        self.draw_cell(maze.start, cell_size, GREEN);
        self.draw_cell(maze.end, cell_size, RED);

        for line_mode in 0..=1 {
            for i in 0..maze.map.len() {
                let pos = vec2(
                    (i % maze.size.x) as f32 * cell_size + self.padding.x,
                    (i / maze.size.x) as f32 * cell_size + self.padding.y,
                );

                let cell = maze.map[i];

                let is_background = line_mode == 0;
                self.draw_wall_line(is_background, pos, cell_size, cell.walls.up, vec2(1., 0.));
                self.draw_wall_line(is_background, pos, cell_size, cell.walls.left, vec2(0., 1.));
                self.draw_wall_line(
                    is_background,
                    pos + vec2(cell_size, 0.),
                    cell_size,
                    cell.walls.right,
                    vec2(0., 1.),
                );
                self.draw_wall_line(
                    is_background,
                    pos + vec2(0., cell_size),
                    cell_size,
                    cell.walls.down,
                    vec2(1., 0.),
                );
            }
        }

        self.fps_history.insert(0, get_fps());
        if self.fps_history.len() > 100 {
            self.fps_history.pop();
        }

        //
        // UI
        //

        // FPS counter
        // draw_text_ex(
        //     &format!(
        //         "FPS: {}",
        //         self.fps_history.iter().sum::<i32>() / self.fps_history.len() as i32
        //     ),
        //     screen_size.x - 80.,
        //     25.0,
        //     TextParams {
        //         font: Some(&self.font),
        //         font_size: 18,
        //         font_scale: 1.,
        //         font_scale_aspect: 1.,
        //         rotation: 0.,
        //         color: WHITE,
        //     },
        // );

        // Pause indicator
        if !maze.play {
            draw_rectangle_lines(screen_size.x / 2. - 6., 12., 10., 30., 8., WHITE);
            draw_rectangle_lines(screen_size.x / 2. + 6., 12., 10., 30., 8., WHITE);
        }
    }

    fn draw_cell(&self, pos: Point, cell_size: f32, color: Color) {
        let screen_pos = vec2(pos.x as f32, pos.y as f32) * cell_size + self.padding;
        draw_rectangle(screen_pos.x, screen_pos.y, cell_size, cell_size, color);
    }

    fn draw_wall_line(
        &self,
        is_background: bool,
        pos: Vec2,
        line_length: f32,
        should_draw: bool,
        dxy: Vec2,
    ) {
        if should_draw {
            if is_background {
                let thickness = self.fill_gap * 2. + self.stroke_width;

                let start = pos - dxy * (self.stroke_width / 2. + self.fill_gap);

                let end = start + dxy * (line_length + self.stroke_width + self.fill_gap * 2.);

                draw_line(start.x, start.y, end.x, end.y, thickness, BLACK);
            } else {
                let start = pos - dxy * (self.stroke_width / 2.);
                let end = start + dxy * (line_length + self.stroke_width);

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
