// game.rs

use crate::snake::Snake;
use crate::ai::find_path;
use crate::high_score;
use rand::Rng;

/// 游戏核心结构，包含蛇、食物、分数、状态及定时器。
pub struct Game {
    pub snake: Snake,
    pub food: (i32, i32),
    pub score: u32,
    pub high_score: u32,
    pub game_over: bool,
    pub won: bool,
    pub timer: f32,
    pub update_interval: f32,
    pub ai_mode: bool,
}

impl Game {
    /// 创建新游戏实例，从已有的最高分初始化。
    pub fn new(high_score: u32) -> Self {
        let mut game = Self {
            snake: Snake::new(),
            food: (0, 0),
            score: 0,
            high_score,
            game_over: false,
            won: false,
            timer: 0.0,
            update_interval: 0.15,
            ai_mode: false,
        };
        game.gen_food();
        game
    }

    /// 生成新食物，优先在内部区域（留出边界安全区），若无法放置则尝试全局，若无空位则判定胜利。
    pub fn gen_food(&mut self) {
        let mut rng = rand::thread_rng();
        // 1. 优先在非边界区域生成（留出一圈安全区）
        for _ in 0..1000 {
            let pos = (
                rng.gen_range(1..(crate::GRID_SIZE - 1)),
                rng.gen_range(1..(crate::GRID_SIZE - 1)),
            );
            if !self.snake.positions().any(|&p| p == pos) {
                self.food = pos;
                return;
            }
        }
        // 2. 如果内部区域已被蛇占满，则允许在边界生成（此时蛇已接近胜利）
        for _ in 0..1000 {
            let pos = (
                rng.gen_range(0..crate::GRID_SIZE),
                rng.gen_range(0..crate::GRID_SIZE),
            );
            if !self.snake.positions().any(|&p| p == pos) {
                self.food = pos;
                return;
            }
        }
        // 3. 无空位 → 胜利
        self.game_over = true;
        self.won = true;
        high_score::save(self.high_score); 
    }

    /// 更新游戏逻辑：计时驱动移动，处理 AI 决策、碰撞检测、得分及食物生成。
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        self.timer += macroquad::prelude::get_frame_time();
        if self.timer >= self.update_interval {
            self.timer -= self.update_interval;

            // AI 模式下自动选择方向
            if self.ai_mode {
                if let Some(dir) = find_path(&self.snake, self.food) {
                    self.snake.set_direction(dir);
                }
            }

            let grow = self.snake.head() == self.food;
            self.snake.step(grow);

            if grow {
                self.score += 1;
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
                self.update_interval = (self.update_interval - 0.003).max(0.08);
                self.gen_food();
                if self.game_over {
                    return;
                }
                crate::sound::play_eat();  // 播放进食音效
            }

            // 撞墙或撞自身检测
            let head = self.snake.head();
            if head.0 < 0 || head.0 >= crate::GRID_SIZE || head.1 < 0 || head.1 >= crate::GRID_SIZE {
                self.game_over = true;
                self.won = false;
                high_score::save(self.high_score);
                crate::sound::play_gameover();  // 播放游戏结束音效
                return;
            }
            if self.snake.positions().skip(1).any(|&p| p == head) {
                self.game_over = true;
                self.won = false;
                high_score::save(self.high_score);
                crate::sound::play_gameover();
                return;
            }
        }
    }

    /// 重置游戏，保留最高分。
    pub fn restart(&mut self) {
        let high = self.high_score;
        *self = Self::new(high);
    }
}
