use animators::{AnimatorFunction, Animators};
use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use core::num;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::time::Instant;

use config::Config;
use terminal_grid::{ TerminalGrid, init_terminal };
use audio_process_buffer::{audio_callback, err_callback, AudioFeatures, AudioProcessBuffer};

pub mod config;
pub mod audio_process_buffer;
pub mod terminal_grid;
pub mod animators;
pub mod colors;

fn main() -> Result<(), anyhow::Error> {
    let config_ini = Config::new("config.ini");
    let animators: Animators = Animators::new(&config_ini);                        
    let mut grid = TerminalGrid::new(config_ini.bg_color);
    let mut audio_features: AudioFeatures;

    if animators.list.len() == 0 {
        return Err(anyhow::Error::msg("Error: no active animations."));
    }

    // Initialize multithreaded access to a shared audio process buffer
    let process_buffer_writer = Arc::new(Mutex::new(AudioProcessBuffer::new()));
    let process_buffer_reader = process_buffer_writer.clone();

    // Audio Initialization 
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find input device");
    let config = device
        .default_output_config()
        .expect("Failed to get default input config");
    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| audio_callback(audio_buffer, &process_buffer_writer),
            move |err| err_callback(err),
            None,
        )?,
        sample_format => {
            return Err(anyhow::Error::msg(format!(
                "Unsupported sample format '{sample_format}'"
            )))
        }
    };
    stream.play()?;

    let animation_duration = config_ini.animation_length as i32;
    let num_animators = animators.list.len() as i32;
    let start = Instant::now();
    loop {
        thread::sleep(time::Duration::from_secs_f32(0.015));

        match process_buffer_reader.try_lock() {
            Ok(buffer) => {
                audio_features = buffer.features;
            },
            Err(_) => {continue;}
        }

        let elapsed = start.elapsed().as_secs_f32();
        let animator_idx = (elapsed as i32/animation_duration) % num_animators;  
        let animator_idx = animator_idx as usize;
        animators.list[animator_idx](&config_ini, &audio_features, elapsed, &mut grid);
        grid.display();
    }

    stream.pause()?;
    drop(stream);
    Ok(())

}
