
use crate::terminal_grid::{TerminalGrid, ColoredChar};
use crate::colors::{COLOR_1, COLOR_2, COLOR_3, COLOR_BG_ALT}; 


pub fn sine_like(local_rms: f32, local_zcr: f32, _elapsed: f32, grid: &mut TerminalGrid) {
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

        grid.draw_string_vertical(j, 0, &s, COLOR_BG_ALT);
        grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_1);
        grid.draw_string_vertical(j, (grid.height-w.len())/2 + w.len(), &s, COLOR_BG_ALT);

        size = size/2;
        w= String::from_utf8(vec![b'*'; size*2]).unwrap();
        grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_2);

        size = size/2;
        w= String::from_utf8(vec![b'*'; size*2]).unwrap();
        grid.draw_string_vertical(j, (grid.height-w.len())/2, &w, COLOR_3);
    }
} 


pub fn wiggly(local_rms: f32, local_zcr: f32, elapsed: f32, grid: &mut TerminalGrid) {
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
            grid.set_cell(i, j, ColoredChar{ c: c, color: col });
        }
    }
}