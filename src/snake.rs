// snake.rs

use std::collections::VecDeque;

/// 蛇的移动方向枚举
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// 蛇结构，使用双端队列存储身体坐标，头部在 front。
#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<(i32, i32)>,
    pub(crate) dir: Direction,
    pub(crate) next_dir: Direction,
}

impl Snake {
    /// 创建初始长度为 3 的蛇，位于网格中央，方向向右。
    pub fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back((10, 10));
        body.push_back((9, 10));
        body.push_back((8, 10));
        Self {
            body,
            dir: Direction::Right,
            next_dir: Direction::Right,
        }
    }

    /// 返回蛇头坐标。
    pub fn head(&self) -> (i32, i32) {
        *self.body.front().unwrap()
    }

    /// 设置下一次移动的方向，禁止直接掉头。
    pub fn set_direction(&mut self, dir: Direction) {
        // 禁止直接掉头
        match (self.dir, dir) {
            (Direction::Up, Direction::Down) => return,
            (Direction::Down, Direction::Up) => return,
            (Direction::Left, Direction::Right) => return,
            (Direction::Right, Direction::Left) => return,
            _ => {}
        }
        self.next_dir = dir;
    }

    /// 执行一步移动，若 grow 为 true 则蛇身增长（吃到食物）。
    pub fn step(&mut self, grow: bool) {
        self.dir = self.next_dir;
        let (x, y) = self.head();
        let new_head = match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };
        self.body.push_front(new_head);
        if !grow {
            self.body.pop_back();
        }
    }

    /// 返回蛇身所有坐标的迭代器，用于碰撞检测。
    pub fn positions(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.body.iter()
    }
}