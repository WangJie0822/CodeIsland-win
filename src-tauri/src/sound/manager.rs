use std::io::Cursor;
use log::warn;

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoundType {
    Approval,
    SessionStart,
    Error,
}

pub struct SoundManager {
    enabled: bool,
}

impl SoundManager {
    pub fn new() -> Self {
        Self { enabled: true }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn play(&self, sound: SoundType) {
        if !self.enabled { return; }

        std::thread::spawn(move || {
            if let Err(e) = play_wav(sound) {
                warn!("[Sound] 播放失败: {}", e);
            }
        });
    }
}

fn play_wav(sound: SoundType) -> Result<(), String> {
    let bytes: &[u8] = match sound {
        SoundType::Approval => include_bytes!("../../resources/sounds/approval.wav"),
        SoundType::SessionStart => include_bytes!("../../resources/sounds/session_start.wav"),
        SoundType::Error => include_bytes!("../../resources/sounds/error.wav"),
    };

    let (_stream, handle) = rodio::OutputStream::try_default()
        .map_err(|e| format!("音频设备: {}", e))?;
    let sink = rodio::Sink::try_new(&handle)
        .map_err(|e| format!("音频 sink: {}", e))?;
    let cursor = Cursor::new(bytes);
    let source = rodio::Decoder::new(cursor)
        .map_err(|e| format!("解码: {}", e))?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
