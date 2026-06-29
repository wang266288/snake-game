// draw.rs

use macroquad::prelude::*;
use crate::Game;
use crate::{GRID_SIZE, CELL_SIZE, WINDOW_SIZE};

/// 绘制游戏界面：网格、食物、蛇身、分数、AI 状态及游戏结束信息。
pub fn draw_game(game: &Game) {
    clear_background(Color::new(0.15, 0.15, 0.15, 1.0));

    // 绘制网格线
    for x in 0..=GRID_SIZE {
        draw_line(
            x as f32 * CELL_SIZE, 0.0,
            x as f32 * CELL_SIZE, WINDOW_SIZE,
            1.0,
            Color::new(0.3, 0.3, 0.3, 0.5),
        );
    }
    for y in 0..=GRID_SIZE {
        draw_line(
            0.0, y as f32 * CELL_SIZE,
            WINDOW_SIZE, y as f32 * CELL_SIZE,
            1.0,
            Color::new(0.3, 0.3, 0.3, 0.5),
        );
    }

    // 绘制食物（红色方块）
    draw_rectangle(
        game.food.0 as f32 * CELL_SIZE,
        game.food.1 as f32 * CELL_SIZE,
        CELL_SIZE, CELL_SIZE,
        RED,
    );

    // 绘制蛇身：头部为深绿色，其余为浅绿色
    for (i, &(x, y)) in game.snake.positions().enumerate() {
        let color = if i == 0 {
            Color::new(0.2, 0.8, 0.2, 1.0)
        } else {
            Color::new(0.4, 1.0, 0.4, 1.0)
        };
        draw_rectangle(
            x as f32 * CELL_SIZE,
            y as f32 * CELL_SIZE,
            CELL_SIZE, CELL_SIZE,
            color,
        );
        draw_rectangle_lines(
            x as f32 * CELL_SIZE,
            y as f32 * CELL_SIZE,
            CELL_SIZE, CELL_SIZE,
            1.0,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
    }

    // 显示当前分数、最高分以及 AI 状态
    draw_text(
        &format!("Score: {}  High: {}", game.score, game.high_score),
        10.0, 30.0, 25.0, WHITE,
    );
    let ai_text = if game.ai_mode { "AI ON" } else { "AI OFF" };
    draw_text(
        ai_text,
        WINDOW_SIZE - 100.0, 30.0,
        20.0,
        if game.ai_mode { GREEN } else { GRAY },
    );

    // 游戏结束时显示获胜或失败信息
    if game.game_over {
        let msg = if game.won {
            "🎉 You Win! Press R to restart"
        } else {
            "💀 Game Over! Press R to restart"
        };
        let size = 35.0;
        let width = measure_text(msg, None, size as u16, 1.0).width;
        draw_text(
            msg,
            (WINDOW_SIZE - width) / 2.0,
            WINDOW_SIZE / 2.0,
            size,
            if game.won { GREEN } else { RED },
        );
    }
}