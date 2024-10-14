use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::time::{Instant};
//use time::Instant;

use terminal_grid::{ TerminalGrid, init_terminal };
use audio_process_buffer::{AudioProcessBuffer, audio_callback, err_callback};
use colors::COLOR_BG;

pub mod audio_process_buffer;
pub mod terminal_grid;
pub mod animators;
pub mod colors;

fn main() -> Result<(), anyhow::Error> {
    // Init audio listening
    let process_buffer_ptr = Arc::new(Mutex::new(AudioProcessBuffer::new()));
    let process_buffer_ptr2 = process_buffer_ptr.clone();
    
    let host = cpal::default_host();
    let device = host.default_output_device()
                     .expect("failed to find input device");
    let config = device
        .default_output_config()
        .expect("Failed to get default input config");

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |audio_buffer, _: &_| audio_callback(audio_buffer, &process_buffer_ptr2),
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

    // Init ASCII grid
    let mut grid = TerminalGrid::new(COLOR_BG);
    init_terminal();
    
    let mut local_zcr = 0.0;
    let mut local_rms = 0.0;

    let start = Instant::now();

    let animators: Vec<fn(f32,f32,f32,&mut TerminalGrid)> = vec![
        animators::sine_like,
        animators::wiggly,
        animators::test,
        //animators::test2
    ];   

    loop {
        thread::sleep(time::Duration::from_secs_f32(0.015));
        match process_buffer_ptr.try_lock() {
            Ok(buffer) => {
                local_zcr = buffer.zcr.smoothed_val;
                local_rms = buffer.rms.smoothed_val;
            },
            Err(_) => {}
        }

        let elapsed = start.elapsed().as_secs_f32();
        let animator_idx = ((elapsed as i32)/4) % (animators.len() as i32);  
        let animator_idx = animator_idx as usize;

        animators[animator_idx](local_rms, local_zcr, elapsed, &mut grid);
        grid.display();
    }

    //stream.pause()?;
    //drop(stream);
    //Ok(())
}
