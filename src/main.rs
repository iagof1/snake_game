#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use eframe::egui;
use egui::*;

const SNAKE_BLOCK_SIZE: f32 = 10.0;
const SNAKE_INITIAL_SIZE: i32 = 4;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "Snake game",
        options,
        Box::new(|_cc| Box::new(SnakeGame::new())),
    )
}

#[derive(PartialEq)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    direction: SnakeDirection,
    body: Vec<Rect>,
}

impl Snake {
    fn new() -> Self {
        let mut body: Vec<Rect> = vec![];

        for i in 0..SNAKE_INITIAL_SIZE {
            let rect = Rect::from_min_max(
                Pos2::new(100.0 + (i as f32 * SNAKE_BLOCK_SIZE), 100.0),
                Pos2::new(
                    100.0 + SNAKE_BLOCK_SIZE + (i as f32 * SNAKE_BLOCK_SIZE),
                    100.0 + SNAKE_BLOCK_SIZE,
                ),
            );
            body.push(rect);
        }

        Self {
            direction: SnakeDirection::Right,
            body,
        }
    }

    fn walk(&mut self) {
        match self.direction {
            SnakeDirection::Up => {
                self.body.remove(0);
                self.body.push(self.body.last().unwrap().translate(Vec2 {
                    x: 0.0,
                    y: -SNAKE_BLOCK_SIZE + 5.,
                }));
            }
            SnakeDirection::Down => {
                self.body.remove(0);
                self.body.push(self.body.last().unwrap().translate(Vec2 {
                    x: 0.0,
                    y: SNAKE_BLOCK_SIZE - 5.,
                }));
            }
            SnakeDirection::Left => {
                self.body.remove(0);
                self.body.push(self.body.last().unwrap().translate(Vec2 {
                    x: -SNAKE_BLOCK_SIZE + 5.,
                    y: 0.0,
                }));
            }
            SnakeDirection::Right => {
                self.body.remove(0);
                self.body.push(self.body.last().unwrap().translate(Vec2 {
                    x: SNAKE_BLOCK_SIZE - 5.,
                    y: 0.0,
                }));
            }
        }
    }

    fn change_direction(&mut self, direction: SnakeDirection) {
        match direction {
            SnakeDirection::Up => {
                if self.direction != SnakeDirection::Down {
                    self.direction = direction;
                }
            }
            SnakeDirection::Down => {
                if self.direction != SnakeDirection::Up {
                    self.direction = direction;
                }
            }
            SnakeDirection::Left => {
                if self.direction != SnakeDirection::Right {
                    self.direction = direction;
                }
            }
            SnakeDirection::Right => {
                if self.direction != SnakeDirection::Left {
                    self.direction = direction;
                }
            }
        }
    }
}

struct SnakeGame {
    snake: Snake,
    game_over: bool,
}

impl SnakeGame {
    fn new() -> Self {
        let snake = Snake::new();
        let game_over = false;
        Self { snake, game_over }
    }
}

impl eframe::App for SnakeGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            std::thread::sleep(Duration::from_millis(20));

            egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
                for rect in &self.snake.body {
                    let square_color = Color32::from_rgb(255, 255, 0);
                    ui.painter().rect(*rect, 1.0, square_color, Stroke::NONE);
                }
            });

            if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                self.snake.change_direction(SnakeDirection::Up);
            } else if ctx.input(|i| i.key_down(Key::ArrowDown)) {
                self.snake.change_direction(SnakeDirection::Down);
            } else if ctx.input(|i| i.key_down(Key::ArrowLeft)) {
                self.snake.change_direction(SnakeDirection::Left);
            } else if ctx.input(|i| i.key_down(Key::ArrowRight)) {
                self.snake.change_direction(SnakeDirection::Right);
            }

            if self.snake.body.first().unwrap().x_range().start().to_owned() >=
            ui.available_size().x ||
            self.snake.body.first().unwrap().y_range().start().to_owned() >=
            ui.available_size().y ||
            self.snake.body.first().unwrap().x_range().start().to_owned() == 0.
            || self.snake.body.first().unwrap().y_range().start().to_owned() == 0. {
                self.game_over = true;
            }

            if self.game_over {
                ui.centered_and_justified(|ui| {
                    ui.heading("Game Over!");
                });
            }

            if !self.game_over {
                self.snake.walk();
            }
        });
    }
}
