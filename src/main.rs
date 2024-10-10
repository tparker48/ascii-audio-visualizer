use anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::time::{Duration, Instant};
//use time::Instant;

use crate::terminal_grid::{TerminalGrid, ColoredChar, init_terminal };
use audio_process_buffer::{AudioProcessBuffer, audio_callback, err_callback};

pub mod audio_process_buffer;
pub mod terminal_grid;

const COLOR_1: (u8,u8,u8) = (230,180,100);
const COLOR_2: (u8,u8,u8) = (237, 110, 88);
const COLOR_3: (u8,u8,u8) = (245, 230, 191);
const COLOR_BG: (u8,u8,u8) = (40,40,40);
const COLOR_BG_DOT: (u8,u8,u8) = (70,70,70);

fn main() -> Result<(), anyhow::Error> {
    //let now = Instant::now();

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

    loop {
        thread::sleep(time::Duration::from_secs_f32(0.015));
        match process_buffer_ptr.try_lock() {
            Ok(buffer) => {
                local_zcr = buffer.zcr.smoothed_val;
                local_rms = buffer.rms.smoothed_val;
            },
            Err(_) => {}
        }

        let elapsed = start.elapsed().as_secs_f32() % 20.0;
        
        if elapsed <= 10.0 {
            for j in 0..grid.width{
                let mut jf = j as f32;
                jf = (jf / (grid.width as f32)) * 720.0;
                jf = jf * (local_zcr+0.01) * 0.4 * (grid.height as f32);
                let sin_out = ((jf*0.04 + 0.8).sin()+1.0)/2.0;
                
                let mut sin_scale = sin_out * 0.028 * (grid.height as f32);
                sin_scale = (15.0*(local_rms * local_rms))*(sin_scale*0.80 + 0.2) +0.5;
                let size = sin_scale as usize;
                let mut size = size.min(grid.height/2);
                let mut w: String = String::from_utf8(vec![b'*'; size*2]).unwrap();
                let mut s: String = String::from_utf8(vec![b'.';(grid.height-w.len()) / 2]).unwrap();
    
                grid.draw_string_vertical(j, 0, &s, COLOR_BG_DOT);
                grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_1);
                grid.draw_string_vertical(j, (grid.height-w.len())/2 + w.len(), &s, COLOR_BG_DOT);
    
                size = size/2;
                w= String::from_utf8(vec![b'*'; size*2]).unwrap();
                grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_2);
    
                size = size/2;
                w= String::from_utf8(vec![b'*'; size*2]).unwrap();
                grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_3);
            }
        } 
        else {
            let center_x = grid.width / 2;
            let center_y = grid.height / 2;
            for i in 0..grid.width{
                for j in 0..grid.height{
                    let dist_x = (i as f32) - (center_x as f32);
                    let dist_y = (j as f32) - (center_y as f32);
                    let dist = (dist_y.powi(2)).sqrt();
                    let mut sin_out = (0.05*(local_zcr*1.8 + 0.2) *dist_y*dist_x + 1.0*elapsed).sin();
                    sin_out = (sin_out + 1.0)/2.0;
                    sin_out = sin_out * (local_rms*local_rms*1.2);
                    let mut col = COLOR_BG_DOT;
                    let mut c = '.';
                    if sin_out > 0.5 {
                        col = COLOR_3;
                        c = '*';
                    } else if sin_out > 0.2 {
                        col = COLOR_BG_DOT;
                        c = '+';
                    } else if sin_out > 0.01 {
                        col = COLOR_BG_DOT;
                        c = '+';
                    }
                    grid.set_cell(i, j, ColoredChar{ c: c, color: col });
                }
            }
        }


        grid.display();
    }

    //stream.pause()?;
    //drop(stream);
    //Ok(())
}