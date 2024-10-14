use crossterm::style::Colored;

use crate::terminal_grid::{TerminalGrid, ColoredChar};
use crate::colors::{color_linterp, BLOCK_CHAR, COLOR_1, COLOR_2, COLOR_3, COLOR_BG, COLOR_BG_ALT}; 


pub fn sine_like(local_rms: f32, local_zcr: f32, _elapsed: f32, grid: &mut TerminalGrid) {
    let center_idx = grid.height/2;
    
    // fill background
    let background_dots: String = String::from_utf8(vec![b'.';grid.height]).unwrap();
    for x in 0..grid.width {
        grid.draw_string_vertical(x, 0, &background_dots, COLOR_BG_ALT);
    }
    
    // draw waves
    for x in 0..grid.width{
        let mut x_position = (x as f32) / (grid.width as f32) ;
        x_position *= (local_zcr+0.01) * 288.0 * (grid.height as f32);
        x_position = (x_position * 0.04) + 0.8;
        
        // sin output is rescaled from [-1,1] to [0,1]
        let mut sin_out = (x_position.sin()+1.0)/2.0;
        sin_out = sin_out * (grid.height as f32) * 0.028;
        sin_out = 15.0* local_rms * local_rms * (sin_out*0.80 + 0.2) + 0.5;
 
        // draw waves 
        let wave_size = 2*(sin_out as usize).min(grid.height/2);
        let wave1: String = String::from_utf8(vec![b'*'; wave_size]).unwrap();
        let wave2: String = String::from_utf8(vec![b'*'; wave_size/2]).unwrap();
        let wave3: String = String::from_utf8(vec![b'*'; wave_size/4]).unwrap();
        grid.draw_string_vertical(x, center_idx-wave1.len()/2, &wave1, COLOR_1);
        grid.draw_string_vertical(x, center_idx-wave2.len()/2, &wave2, COLOR_2);
        grid.draw_string_vertical(x, center_idx-wave3.len()/2, &wave3, COLOR_3);
    }
} 

pub fn wiggly(local_rms: f32, local_zcr: f32, elapsed: f32, grid: &mut TerminalGrid) {
    let center_x = grid.width / 2;
    let center_y = grid.height / 2;
    for i in 0..grid.width{
        for j in 0..grid.height{
            let dist_x = (i as f32) - (center_x as f32);
            let dist_y = (j as f32) - (center_y as f32);
            let mut sin_out = (0.05*(local_zcr*1.8 + 0.2) *dist_y*dist_x + 1.0*elapsed).sin();
            sin_out = (sin_out + 1.0)/2.0;
            sin_out = sin_out * (local_rms*local_rms*1.2);
            let mut col = COLOR_BG_ALT;
            let mut c = '.';
            if sin_out > 0.5 {
                col = COLOR_1;
                c = '*';
            } else if sin_out > 0.2 {
                col = COLOR_BG_ALT;
                c = '+';
            } else if sin_out > 0.01 {
                col = COLOR_BG_ALT;
                c = '+';
            }
            grid.set_cell(i, j, ColoredChar{ c: c, color: col });
        }
    }
}

pub fn test(rms: f32, zcr: f32, elapsed: f32, grid: &mut TerminalGrid) {
    let x_pad = 4;
    let y_pad = 2;
    grid.fill(ColoredChar{ c:'.', color: COLOR_BG_ALT });
    grid.draw_box(
        x_pad, 
        y_pad, 
        grid.width - 2*x_pad, 
        grid.height - 2*y_pad, 
        ColoredChar{c:'x', color:COLOR_2}
    );

    for i in x_pad..grid.width-x_pad{
        let mut sin_out =   ((2.0*elapsed + 0.1*(i as f32)).sin()/2.0 + 0.5);
        sin_out = sin_out*0.7 + 0.3;
        sin_out *= rms * rms; 
        sin_out = 0.8*((grid.height - 2*y_pad) as f32) * sin_out;
        let sin_height = sin_out as usize;
        let bar = String::from_utf8(vec![b'.'; sin_height]).unwrap();
        grid.draw_string_vertical(i, (grid.height - bar.len())/2, &bar, COLOR_2);
    }

}

pub fn test2(rms: f32, zcr: f32, elapsed: f32, grid: &mut TerminalGrid) {
    let x_pad = 4;
    let y_pad = 2;
    let color = COLOR_2;

    for i in x_pad..grid.width-x_pad-1 {
        for j in y_pad..grid.height-y_pad{
            grid.set_cell(i, j, grid.get_cell(i+1, j).clone());
        }
    }


    let i = grid.width - x_pad - 1;
    let sin_height = (rms * rms * ((grid.height/3) as f32)) as usize;
    let bar = String::from_utf8(vec![b'.'; sin_height]).unwrap();
    let mt = String::from_utf8(vec![b' '; grid.height]).unwrap();
    let mut sin_height2 = ((zcr*8.0) * ((grid.height/3) as f32)) as usize;
    sin_height2 = sin_height2.min(grid.height/3);
    let bar2 = String::from_utf8(vec![b'.'; sin_height2]).unwrap();
    grid.draw_string_vertical(i, 0, &mt, COLOR_BG);
    grid.draw_string_vertical(i, grid.height/3 - bar.len()/2, &bar, color);
    grid.draw_string_vertical(i, 2*grid.height/3 - bar2.len()/2, &bar2, COLOR_1);

}