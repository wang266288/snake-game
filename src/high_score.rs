// high_score.rs

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const SCORE_FILE: &str = "high_score.json";

#[derive(Serialize, Deserialize)]
struct ScoreData {
    high_score: u32,
}

/// 从文件加载最高分，若文件不存在则返回 0。
pub fn load() -> u32 {
    if !Path::new(SCORE_FILE).exists() {
        return 0;
    }
    if let Ok(data) = fs::read_to_string(SCORE_FILE) {
        if let Ok(score_data) = serde_json::from_str::<ScoreData>(&data) {
            return score_data.high_score;
        }
    }
    0
}

/// 将最高分保存到文件。
pub fn save(high_score: u32) {
    let data = ScoreData { high_score };
    let _ = fs::write(SCORE_FILE, serde_json::to_string_pretty(&data).unwrap());
}