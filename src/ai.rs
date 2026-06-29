// ai.rs

use crate::snake::{Direction, Snake};
use crate::GRID_SIZE;
use std::collections::VecDeque;

/// 主决策入口：优先尝试 BFS 最短路径，若路径安全性不足则使用评分函数选择最佳方向。
/// 返回可选的最优移动方向。
pub fn find_path(snake: &Snake, food: (i32, i32)) -> Option<Direction> {
    // 1. 尝试 BFS 路径
    if let Some(dir) = bfs(snake, food) {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_head = (snake.head().0 + dx, snake.head().1 + dy);
        let will_eat = new_head == food;
        let area = count_reachable_area(snake, new_head, will_eat);
        // 必须安全且可达面积至少为蛇身长度的 2 倍
        if is_move_safe(snake, dir, Some(food)) && area >= snake.body.len() * 2 {
            return Some(dir);
        }
    }

    // 2. 若 BFS 方向不合格，则用评分选择最佳方向
    let head = snake.head();
    let dirs = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut best_dir = None;
    let mut best_score = f64::NEG_INFINITY;

    for &dir in &dirs {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        let new_head = (head.0 + dx, head.1 + dy);
        if !is_move_safe(snake, dir, Some(food)) {
            continue;
        }
        let will_eat = new_head == food;
        let area = count_reachable_area(snake, new_head, will_eat);
        let dist_food = (new_head.0 - food.0).abs() + (new_head.1 - food.1).abs();
        let tail = *snake.body.back().unwrap();
        let dist_tail = (new_head.0 - tail.0).abs() + (new_head.1 - tail.1).abs();

        let mut score = 0.0;

        // 面积越大越安全（主要指标）
        score += area as f64 * 2.0;

        // 如果面积太小，严厉惩罚
        if area < 10 {
            score -= 1000.0;
        }

        // 吃到食物奖励（适度）
        if will_eat {
            score += 20.0;
        }

        // 离食物越近越好，但权重降低（避免贪心）
        score -= dist_food as f64 * 0.2;

        // 离蛇尾越近越好，权重提高（关键安全策略）
        score -= dist_tail as f64 * 1.5;

        if score > best_score {
            best_score = score;
            best_dir = Some(dir);
        }
    }

    best_dir
}

/// BFS 搜索从蛇头到食物的最短路径，蛇尾视为可通行（因为移动后蛇尾会移开）。
/// 返回到达食物的第一步方向，若不可达则返回 None。
fn bfs(snake: &Snake, food: (i32, i32)) -> Option<Direction> {
    let head = snake.head();
    let body: Vec<_> = snake.body.iter().cloned().collect();
    let tail = *body.last().unwrap();

    let mut occupied = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
    for &(x, y) in &body {
        if (x, y) != tail {
            occupied[y as usize][x as usize] = true;
        }
    }

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
    let mut parent = vec![vec![None; GRID_SIZE as usize]; GRID_SIZE as usize];

    queue.push_back(head);
    visited[head.1 as usize][head.0 as usize] = true;

    let dirs = [
        (0, -1, Direction::Up),
        (0, 1, Direction::Down),
        (-1, 0, Direction::Left),
        (1, 0, Direction::Right),
    ];

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == food {
            let mut cur = (x, y);
            while let Some((px, py, dir)) = parent[cur.1 as usize][cur.0 as usize] {
                if (px, py) == head {
                    return Some(dir);
                }
                cur = (px, py);
            }
            return None;
        }

        for (dx, dy, dir) in &dirs {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= GRID_SIZE || ny < 0 || ny >= GRID_SIZE {
                continue;
            }
            if visited[ny as usize][nx as usize] || occupied[ny as usize][nx as usize] {
                continue;
            }
            visited[ny as usize][nx as usize] = true;
            parent[ny as usize][nx as usize] = Some((x, y, *dir));
            queue.push_back((nx, ny));
        }
    }
    None
}

/// 计算从 new_head 出发，在模拟移动后的蛇身障碍下可达的格子数量（即“安全区域”大小）。
/// 参数 will_eat 表示这一步是否会吃到食物，影响蛇身是否缩短。
fn count_reachable_area(snake: &Snake, new_head: (i32, i32), will_eat: bool) -> usize {
    // 模拟移动后的蛇身
    let mut new_body = snake.body.clone();
    new_body.push_front(new_head);
    if !will_eat {
        new_body.pop_back();
    }
    let body_vec: Vec<_> = new_body.into_iter().collect();

    // 障碍：除新蛇头外的蛇身
    let mut obstacles = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
    for &(x, y) in &body_vec {
        if (x, y) != new_head {
            obstacles[y as usize][x as usize] = true;
        }
    }

    // BFS 从 new_head 出发
    let mut visited = vec![vec![false; GRID_SIZE as usize]; GRID_SIZE as usize];
    let mut queue = VecDeque::new();
    queue.push_back(new_head);
    visited[new_head.1 as usize][new_head.0 as usize] = true;
    let mut count = 0;

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((x, y)) = queue.pop_front() {
        count += 1;
        for (dx, dy) in &dirs {
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || nx >= GRID_SIZE || ny < 0 || ny >= GRID_SIZE {
                continue;
            }
            if obstacles[ny as usize][nx as usize] {
                continue;
            }
            if !visited[ny as usize][nx as usize] {
                visited[ny as usize][nx as usize] = true;
                queue.push_back((nx, ny));
            }
        }
    }
    count
}

/// 安全检测：模拟一次移动，判断是否撞墙或撞身（考虑蛇尾是否移开）。
/// food 参数用于判断是否会吃到食物，从而影响蛇尾是否保留。
fn is_move_safe(snake: &Snake, dir: Direction, food: Option<(i32, i32)>) -> bool {
    let head = snake.head();
    let (dx, dy) = match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };
    let new_head = (head.0 + dx, head.1 + dy);

    if new_head.0 < 0 || new_head.0 >= GRID_SIZE || new_head.1 < 0 || new_head.1 >= GRID_SIZE {
        return false;
    }

    let will_eat = food.map_or(false, |f| new_head == f);
    let body: Vec<_> = snake.body.iter().cloned().collect();
    let tail = *body.last().unwrap();

    for (_i, &pos) in body.iter().enumerate() {
        if pos == head {
            continue;
        }
        if pos == tail && !will_eat {
            continue;
        }
        if pos == new_head {
            return false;
        }
    }
    true
}