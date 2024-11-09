use ansi_term::Color::RGB;
use hex::decode;

pub const BLOCK_CHAR: char = '\u{2588}';

pub type Color = (u8, u8, u8);

pub trait FromHex: Sized {
    fn from_hex_string(hex_str: String) -> Result<Self, String>;
}

impl FromHex for Color {
    fn from_hex_string(hex_str: String) -> Result<Color, String> {
        if !hex_str.starts_with("0x") {
            return Err("Invalid Hex String: Did not start with 0x.".to_string());
        }
        let hex_str = hex_str[2..hex_str.len()].to_string();

        if hex_str.len() != 6 {
            return Err("Invalid Hex String: Invalid length".to_string());
        }
        let hex_value: Vec<u8> = decode(hex_str).expect("Invalid Hex");
        Ok((hex_value[0], hex_value[1], hex_value[2]))
    }
}

#[derive(Copy, Clone)]
pub struct ColoredChar {
    pub c: char,
    pub color: (u8, u8, u8),
}

impl ColoredChar {
    pub fn new(character: char, color: Color) -> ColoredChar {
        ColoredChar {
            c: character,
            color,
        }
    }
    pub fn to_string(self: &ColoredChar, bg_color: Color) -> String {
        let color = RGB(self.color.0, self.color.1, self.color.2);
        let bg_color = RGB(bg_color.0, bg_color.1, bg_color.2);
        color.on(bg_color).paint(self.c.to_string()).to_string()
    }
}

impl PartialEq for ColoredChar {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.color == other.color
    }
}
