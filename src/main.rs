// main.rs

mod snake;
mod game;
mod draw;
mod ai;
mod sound;
mod high_score;

use macroquad::prelude::*;
use game::Game;
use draw::draw_game;

// 游戏常量
pub const GRID_SIZE: i32 = 20;          // 网格尺寸（20x20）
pub const CELL_SIZE: f32 = 30.0;        // 每格像素大小
pub const WINDOW_SIZE: f32 = GRID_SIZE as f32 * CELL_SIZE;

#[macroquad::main("🐍 Snake Game")]
async fn main() {
    request_new_screen_size(WINDOW_SIZE, WINDOW_SIZE);

    // 加载历史最高分
    let high = high_score::load();
    let mut game = Game::new(high);

    loop {
        // ---- 键盘输入处理 ----
        if is_key_pressed(KeyCode::Up) {
            game.snake.set_direction(snake::Direction::Up);
        } else if is_key_pressed(KeyCode::Down) {
            game.snake.set_direction(snake::Direction::Down);
        } else if is_key_pressed(KeyCode::Left) {
            game.snake.set_direction(snake::Direction::Left);
        } else if is_key_pressed(KeyCode::Right) {
            game.snake.set_direction(snake::Direction::Right);
        } else if is_key_pressed(KeyCode::R) && game.game_over {
            // 重置时保存最高分
            high_score::save(game.high_score);
            game.restart();
        } else if is_key_pressed(KeyCode::A) {
            // 切换 AI 模式
            game.ai_mode = !game.ai_mode;
        }

        // ---- 更新游戏（内部触发音效） ----
        game.update();

        // ---- 绘制界面 ----
        draw_game(&game);
        next_frame().await;
    }
}