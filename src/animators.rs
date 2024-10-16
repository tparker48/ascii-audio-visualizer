use crate::audio_process_buffer::AudioFeatures;
use crate::terminal_grid::{TerminalGrid, CC};
use crate::colors::{ BLOCK_CHAR, COLOR_1, COLOR_2, COLOR_3, COLOR_BG_ALT}; 


pub fn sine_like(features: &AudioFeatures, _elapsed: f32, grid: &mut TerminalGrid) {
    let rms = features.root_mean_squared.smoothed_val;
    let zcr = features.zero_crossing_rate.smoothed_val;
    
    let center_idx = grid.height/2;
    
    // fill background
    for x in 0..grid.width {
        grid.draw_line_vertical(
            '.',
            COLOR_BG_ALT,
            x, 
            0, 
            grid.height);
    }
    
    // draw waves
    for x in 0..grid.width{
        let mut x_position = (x as f32) / (grid.width as f32) ;
        x_position *= (zcr+0.01) * 288.0 * (grid.height as f32);
        x_position = (x_position * 0.04) + 0.8;
        
        // sin output is rescaled from [-1,1] to [0,1]
        let mut sin_out = (x_position.sin()+1.0)/2.0;
        sin_out = sin_out * (grid.height as f32) * 0.028;
        sin_out = 15.0* rms * rms * (sin_out*0.80 + 0.2) + 0.5;
 
        // draw waves 
        let wave_size = 2*(sin_out as usize).min(grid.height/2);
        grid.draw_line_vertical(
            '*',
            COLOR_1,
            x, 
            center_idx-wave_size, 
            wave_size);
        grid.draw_line_vertical(
            '*', 
            COLOR_2, 
            x, 
            center_idx-wave_size/2, 
            wave_size/2);
        grid.draw_line_vertical(
            '*', 
            COLOR_3, 
            x, 
            center_idx-wave_size/4, 
            wave_size/4);
    }
} 

pub fn wiggly(features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
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
            let mut col = COLOR_BG_ALT;
            let mut c = '.';
            if sin_out > 0.5 {
                col = COLOR_3;
                c = '*';
            } else if sin_out > 0.2 {
                col = COLOR_BG_ALT;
                c = '+';
            } else if sin_out > 0.01 {
                col = COLOR_BG_ALT;
                c = '+';
            }
            grid.set_cell(c, col, i, j);
        }
    }
}

pub fn wip(features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
    let rms = features.root_mean_squared.smoothed_val;
    let x_pad = 4;
    let y_pad = 2;
    
    grid.fill('.', COLOR_BG_ALT);
    grid.draw_box(
        'x',
        COLOR_2,
        x_pad, 
        y_pad, 
        grid.width - 2*x_pad, 
        grid.height - 2*y_pad, 
    );

    for i in x_pad..grid.width-x_pad{
        let mut sin_out =   ((2.0*elapsed + 0.1*(i as f32)).sin()/2.0 + 0.5);
        sin_out = sin_out*0.7 + 0.3;
        sin_out *= rms*rms; 
        sin_out = 0.8*((grid.height - 2*y_pad) as f32) * sin_out;
        let sin_height = sin_out as usize;
        grid.draw_line_vertical(
            '.',
            COLOR_BG_ALT, 
            i, 
            (grid.height - sin_height)/2, 
            sin_height
        );
    }

}

pub fn test(features: &AudioFeatures, elapsed: f32, grid: &mut TerminalGrid) {
    let bins = features.fft_bins;
    grid.reset();
    for i in 0..bins.len() {
        if i >= grid.width {
            continue;
        }
        let mut bin_height = (bins[i].smoothed_val * 1.5) as usize;
        bin_height = bin_height.min(grid.height);
        let color = (255,0,0);
        grid.draw_line(
            BLOCK_CHAR, 
            color, 
            i, 
            grid.height-1, 
            0, 
            -1, 
            bin_height
        );
    }
}
