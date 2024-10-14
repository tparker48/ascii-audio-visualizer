pub type Color = (u8,u8,u8);

pub const COLOR_1: Color = (230,180,100);
pub const COLOR_2: Color = (237,110,88);
pub const COLOR_3: Color = (245,230,191);
pub const COLOR_BG: Color = (40,40,40);
pub const COLOR_BG_ALT: Color = (70,70,70);

pub const BLOCK_CHAR: char = '\u{2588}';


pub fn color_linterp(c0: Color, c1: Color, mix: f32) -> Color {
    let c0 = (c0.0 as f32, c0.1 as f32, c0.2 as f32);
    let c1 = (c1.0 as f32, c1.1 as f32, c1.2 as f32);
    let m1 = mix;
    let m2 = 1.0-mix;
    let r = (m1*c0.0 + m2*c1.0) as u8;
    let g = (m1*c0.1 + m2*c1.1) as u8;
    let b = (m1*c0.2 + m2*c1.2) as u8;
    return (r,g,b);
}