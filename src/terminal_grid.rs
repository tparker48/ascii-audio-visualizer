use std::io;
use ansi_term::Color::RGB;
use crossterm::execute;
use crossterm::style::{Print};
use crossterm::cursor::{MoveTo, Hide};
use crossterm::terminal::{SetSize, BeginSynchronizedUpdate, EndSynchronizedUpdate};

pub const WIDTH: usize = 120;
pub const HEIGHT: usize = 32;
const GRID_SIZE: usize = WIDTH*HEIGHT;
const BLOCK_CHAR: char = '\u{2588}';

pub fn init_terminal() {
    execute!(
        io::stdout(),
        //EnterAlternateScreen,
        SetSize(WIDTH as u16, HEIGHT as u16),
        Hide
    ).unwrap();
    print!("\x1B[2J");
}

pub struct TerminalGrid {
    grid: [ColoredChar; GRID_SIZE],
    last_grid: [ColoredChar; GRID_SIZE],
    bg_color: ansi_term::Color
}

impl TerminalGrid {
    pub fn new(bg_color: (u8,u8,u8)) -> TerminalGrid {
        TerminalGrid {
            grid : [ ColoredChar{c:BLOCK_CHAR, color:(255,255,255),} ; GRID_SIZE ],
            last_grid : [ ColoredChar{c:BLOCK_CHAR, color:(255,255,255),} ; GRID_SIZE ],
            bg_color: RGB(bg_color.0, bg_color.1, bg_color.2)
        }
    }

    pub fn index_2d(self: & TerminalGrid, i: usize, j: usize) -> usize {
        return j*WIDTH+i;
    }
    
    pub fn set_cell(self: &mut TerminalGrid, i: usize, j:usize, c: ColoredChar) {
        self.grid[self.index_2d(i, j)] = c;
    }

    pub fn get_cell(self: & TerminalGrid, i:usize, j:usize) -> ColoredChar {
        return self.grid[self.index_2d(i, j)];
    }

    pub fn get_line(self: & TerminalGrid, j:usize) -> String {
        let start_idx = self.index_2d(0, j);
        self.grid[start_idx..start_idx+WIDTH]
            .iter()
            .map(|cc| cc.to_string(&self.bg_color))
            .collect()
    }

    pub fn get_lines(self: & TerminalGrid) -> String {
        let mut result = String::from("");
        for (i,line) in self.grid.chunks(WIDTH).enumerate(){
            let joined_line: String = line.iter()
                                           .map( |colored_char| colored_char.to_string(&self.bg_color) )
                                           .collect();
            result.push_str(&joined_line);

            if i < HEIGHT-1{
                result.push('\n');
            }
        }
        return result;
    }

    pub fn draw_string_horizontal(self: &mut TerminalGrid, x:usize, y:usize, string: &String, color: (u8,u8,u8)) {
        let y = y.min(HEIGHT).max(0);
        for i in 0..string.len().min(WIDTH){
            self.set_cell(x + i, y, ColoredChar{c:string.as_bytes()[i] as char, color: color} );
        }
    }

    pub fn draw_string_vertical(self: &mut TerminalGrid, x:usize, y:usize, string: &String, color: (u8,u8,u8)) {
        let x = x.min(WIDTH).max(0);

        for i in 0..string.len().min(HEIGHT){
            self.set_cell(x, y + i, ColoredChar{c:string.as_bytes()[i] as char, color: color} );
        }
    }

    pub fn draw_box(self: &mut TerminalGrid, x:usize, y:usize, w:usize, h:usize, c: ColoredChar) {
        for i in x..(x+w).min(WIDTH) {
            for j in y..(y+h).min(HEIGHT) {
                self.set_cell(i, j, c);
            }
        }
    }

    pub fn reset(self: &mut TerminalGrid) {
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self.set_cell(i, j, ColoredChar{ c:' ', color:(0,0,0)});
                
            }
        }
    }

    pub fn display(self: &mut TerminalGrid) {
        // Detect diffs and copy into "last_grid"
        let mut change = false;
        for i in 0..GRID_SIZE{
            if self.grid[i] != self.last_grid[i] {
                self.last_grid[i] = self.grid[i];
                change = true;
            }
        }
        
        // Be lazy if no changes
        if !change {
            return;
        }
        execute!(
            io::stdout(),
            BeginSynchronizedUpdate,
            MoveTo(0, 0),
            Print(self.get_lines()),
            Hide,
            EndSynchronizedUpdate
        ).unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct ColoredChar {
    pub c: char,
    pub color: (u8,u8,u8)
}

impl ColoredChar {
    fn to_string(self: &ColoredChar, bg_color: &ansi_term::Color) -> String {
        let (r,g,b) = self.color;
        RGB(r,g,b).on(*bg_color).paint(self.c.to_string()).to_string()
    }
}

impl PartialEq for ColoredChar {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.color == other.color
    }
}
