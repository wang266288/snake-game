// sound.rs

use rodio::{OutputStream, Sink, Source};
use rodio::source::SineWave;
use std::time::Duration;

/// 播放“吃食物”音效：440Hz 短促蜂鸣（150ms）。
pub fn play_eat() {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            let source = SineWave::new(440.0)
                .amplify(0.2)
                .take_duration(Duration::from_millis(150));
            let _ = sink.append(source);
            sink.sleep_until_end();
        }
    }
}

/// 播放“游戏结束”音效：220Hz 低沉长音（400ms）。
pub fn play_gameover() {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            let source = SineWave::new(220.0)
                .amplify(0.3)
                .take_duration(Duration::from_millis(400));
            let _ = sink.append(source);
            sink.sleep_until_end();
        }
    }
}