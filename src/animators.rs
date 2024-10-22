use std::collections::HashMap;

use crate::audio_process_buffer::{bin_idx_to_freq, AudioFeatures};
use crate::config::Config;
use crate::terminal_grid::TerminalGrid;

pub type AnimatorFunction = fn(&Config, &AudioFeatures, f32, &mut TerminalGrid);


pub struct Animators {
    pub list: Vec<AnimatorFunction>
}
impl Animators {
    pub fn new(config: &Config) -> Animators {
        let animators: Vec<AnimatorFunction> = config.animations
                                .iter()
                                .map(|name|match_animator(name))
                                .collect();
        return Animators { list: animators }
    }
}


fn match_animator(animator_name: &str) -> AnimatorFunction {
    match animator_name {
        "sine_like" => sine_like,
        "spectrum" => spectrum,
        "wiggly" => wiggly,
        "eq_mountains" => eq_mountains,
        _ => sine_like
    }
} 

pub fn sine_like(config: &Config, features: &AudioFeatures, _elapsed: f32, grid: &mut TerminalGrid) {
    let rms = features.root_mean_squared.smoothed_val;
    let zcr = features.zero_crossing_rate.smoothed_val;
    
    let center_idx = grid.height/2;
    
    // fill background
    for x in 0..grid.width {
        grid.draw_line_vertical(
            '.',
            config.bg_alt_color,
            x, 
            0, 
            grid.height);
    }
    
    // draw waves
    for x in 0..grid.width{
        let mut x_position = (x as f32) / (grid.width as f32) ;
        x_position *= (zcr+0.01) * 288.0 * (grid.height as f32);
        x_position = (x_position * 0.03) + 0.8;
        
        // sin output is rescaled from [-1,1] to [0,1]
        let mut sin_out = (x_position.sin()+1.0)/2.0;
        sin_out = sin_out * (grid.height as f32) * 0.028;
        sin_out = 15.0* rms * rms * (sin_out*0.80 + 0.2) + 0.5;
 
        // draw waves 
        let wave_size = 2*(sin_out as usize).min(center_idx);
        grid.draw_line_vertical(
            '*',
            config.color_1,
            x, 
            center_idx-wave_size/2, 
            wave_size);
        grid.draw_line_vertical(
            '*', 
            config.color_2, 
            x, 
            center_idx-wave_size/4, 
            wave_size/2);
        grid.draw_line_vertical(
            '*', 
            config.color_3, 
            x, 
            center_idx-wave_size/8, 
            wave_size/4);
    }
} 

pub fn wiggly(config: &Config, features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
    let rms = features.root_mean_squared.smoothed_val;
    let zcr = features.zero_crossing_rate.smoothed_val;

    let center_x = grid.width / 2;
    let center_y = grid.height / 2;
    for i in 0..grid.width{
        for j in 0..grid.height{
            let dist_x = (i as f32) - (center_x as f32);
            let dist_y = (j as f32) - (center_y as f32);
            let mut sin_out = (0.05*(zcr*1.8 + 0.2) *dist_y*dist_x + 1.0*elapsed).sin();
            sin_out = (sin_out + 1.0)/2.0;
            sin_out = sin_out * (rms*rms*1.2);
            let mut col = config.bg_alt_color;
            let mut c = '.';
            if sin_out > 0.5 {
                col = config.color_3;
                c = '*';
            } else if sin_out > 0.2 {
                col = config.bg_alt_color;
                c = '+';
            } else if sin_out > 0.01 {
                col = config.bg_alt_color;
                c = '+';
            }
            grid.set_cell(c, col, i, j);
        }
    }
}

pub fn eq_mountains(config: &Config, features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
    let rms = features.root_mean_squared.smoothed_val;

    // scaling lo/mi/hi so they don't overlap
    let lo = features.lo.smoothed_val * 0.9 * rms;
    let mi = features.mi.smoothed_val * 0.3 * rms;
    let hi = features.hi.smoothed_val * 2.0 * rms;

    fn char_height(num: f32, max_height: usize) -> usize {
        ((num * (max_height as f32)) as usize).min(max_height)
    }

    for i in 0..grid.width-1 {
        for j in 0..grid.height {
            grid.set_cell(grid.get_cell(i+1,j).c, grid.get_cell(i+1,j).color, i, j);
        } 
    }
    grid.draw_line_vertical(' ', config.bg_color, grid.width-1, 0, grid.height);
    grid.draw_line('/', config.color_3, grid.width-1, grid.height-1, 0, -1, char_height(hi, grid.height));
    grid.draw_line('\\', config.color_2, grid.width-1, grid.height-1, 0, -1, char_height(mi, grid.height));
    grid.draw_line('/', config.color_1, grid.width-1, grid.height-1, 0, -1, char_height(lo, grid.height));

    for j in 0..grid.height{
        if grid.get_cell(grid.width-1, j).c == ' ' && j%3==0{
            grid.set_cell('.', config.bg_alt_color, grid.width-1, j);
        }
    }
}

pub fn spectrum(config: &Config, features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
    grid.fill('.', config.bg_alt_color);

    // convert FFT bins to log-scaled frequency to magnitude map
    let freq_spectrum: Vec<(f32,f32)> = 
        features.fft_bins
                .iter()
                .enumerate()
                .map(|(bin_idx, sv)| {
                    (bin_idx_to_freq(bin_idx), sv.smoothed_val)
                })
                .filter(|(freq,_mag)| *freq >= 12.0 && *freq <= 10000.0)
                .map(|(freq, mag)|{
                    (freq.log2(), (15.0*mag).log10())
                })       
                .collect();
    let max_freq = freq_spectrum[freq_spectrum.len()-1].0;
    let min_freq = freq_spectrum[0].0;
    let col_width = (max_freq-min_freq) / (grid.width as f32);
    let mut heights = vec![0.0; grid.width];
    for i in 0..grid.width {
        let range_start = min_freq + col_width * (i as f32);
        let range_end = range_start + col_width;
        let mut hits = 0;
        for (freq,magnitude) in freq_spectrum.iter() {
            if range_start <= *freq && *freq < range_end {
                heights[i] += *magnitude;
                hits += 1;
            }
        }
        heights[i] /= hits as f32;
    }

    let cutoff = 0.1;
    let mut last_nonzero_l = 0.0;
    let mut last_nonzero_r = 0.0;
    for i in 0..grid.width{
        if heights[i] > cutoff {
            last_nonzero_l = heights[i];
        } else {
            heights[i] = last_nonzero_l;
        }
        
        last_nonzero_l *= 0.7;
        
    }
    for i in 0..grid.width {
        let rev_i = grid.width - i - 1;
        if heights[rev_i] > cutoff {
            last_nonzero_r = heights[rev_i];
        } else {
            heights[rev_i] = last_nonzero_r;
        }
        last_nonzero_r *= 0.7;

    }

    for i in 0..grid.width {
        let col_height = ((heights[i]*(grid.height as f32)) as usize).min(grid.height);
        let char = '=';
        let color = config.color_1;
        let x = i;
        let y = grid.height -1;
        grid.draw_line(char, color, x, y, 0, -1, col_height);
    } 
}
