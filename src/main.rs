use animators::Animators;
use std::time::Instant;
use std::{thread, time};

use audio_processing::AudioFeatures;
use config::Config;
use terminal_grid::TerminalGrid;

pub mod animators;
pub mod audio_formats;
pub mod audio_processing;
pub mod colors;
pub mod config;
pub mod input;
pub mod terminal_grid;

fn main() -> Result<(), anyhow::Error> {
    let config = Config::load_config();
    let animators: Animators = Animators::new(&config);
    let mut grid = TerminalGrid::new(config.bg_color);
    let mut audio_features: AudioFeatures;

    if animators.list.is_empty() {
        return Err(anyhow::Error::msg("Error: no active animations."));
    }

    // Listen to audio via pulseaudio API on linux.
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd"
    ))]
    let process_buffer_reader = input::pulse::connect().expect("Failed to connect audio listener");

    // Listen to audio via CPAL crate on windows.
    #[cfg(target_os = "windows")]
    let (process_buffer_reader, _stream) =
        input::wasapi::connect().expect("Failed to connect audio listener");

    let animation_duration = config.animation_length as i32;
    let num_animators = animators.list.len() as i32;
    let start = Instant::now();
    let mut elapsed: f32;
    loop {
        thread::sleep(time::Duration::from_secs_f32(0.015));

        match process_buffer_reader.try_lock() {
            Ok(buffer) => {
                audio_features = buffer.features;
            }
            Err(_) => {
                continue;
            }
        }

        elapsed = start.elapsed().as_secs_f32();
        let animator_idx = (elapsed as i32 / animation_duration) % num_animators;
        let animator_idx = animator_idx as usize;
        animators.list[animator_idx](&config, &audio_features, elapsed, &mut grid);
        grid.display();
    }
}
