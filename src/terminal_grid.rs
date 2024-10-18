use std::io;
use ansi_term::Color::RGB;
use crossterm::execute;
use crossterm::style::Print;
use crossterm::cursor::{MoveTo, Hide};
use crossterm::terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate};

use crate::colors::{BLOCK_CHAR, COLOR_BG};

use crate::colors::Color;


pub fn init_terminal() {
    execute!(
        io::stdout(),
        Hide
    ).unwrap();
    print!("\x1B[2J");
}

pub struct TerminalGrid {
    grid: Vec<ColoredChar>, 
    last_grid: Vec<ColoredChar>,
    pub width: usize,
    pub height: usize,
    pub grid_size: usize
}

impl TerminalGrid {
    pub fn new(bg_color: Color) -> TerminalGrid {
        let (w,h) = crossterm::terminal::size().unwrap();
        let w = w as usize;
        let h = h as usize;
        let grid_size = w*h;
        TerminalGrid {
            grid : vec![ColoredChar{c:BLOCK_CHAR, color:bg_color} ; grid_size],
            last_grid : vec![ ColoredChar{c:BLOCK_CHAR, color:(255,255,255),} ; grid_size],
            width: w,
            height: h,
            grid_size: grid_size
        }
    }

    pub fn index_2d(self: & TerminalGrid, i: usize, j: usize) -> usize {
        return j*self.width+i;
    }
    
    pub fn set_cell(self: &mut TerminalGrid, c: char, color: Color, i: usize, j:usize) {
        let t = self.index_2d(i, j);
        if t > self.grid_size {
            return;
        }
        self.grid[t]= ColoredChar{c:c, color:color};
    }

    pub fn get_cell(self: & TerminalGrid, i:usize, j:usize) -> ColoredChar {
        return self.grid[self.index_2d(i, j)];
    }

    pub fn get_line(self: & TerminalGrid, j:usize) -> String {
        let start_idx = self.index_2d(0, j);
        self.grid[start_idx..start_idx+self.width]
            .iter()
            .map(|cc| cc.to_string(COLOR_BG))
            .collect()
    }

    pub fn get_lines(self: & TerminalGrid) -> String {
        let mut result = String::from("");
        for (i,line) in self.grid.chunks(self.width).enumerate(){
            let joined_line: String = line.iter()
                                           .map( |colored_char| colored_char.to_string(COLOR_BG) )
                                           .collect();
            result.push_str(&joined_line);

            if i < self.height-1{
                result.push('\n');
            }
        }
        return result;
    }

    pub fn draw_line(&mut self, c: char, color: Color, x: usize, y: usize, dx: i16, dy: i16, reps: usize) {
        let mut pos_x = x as i16;
        let mut pos_y = y as i16;
        for _ in 0..reps{
            self.set_cell(c, color, pos_x as usize, pos_y as usize);
            pos_x += dx;
            pos_y += dy;
        }
    }

    pub fn draw_line_horizontal(self: &mut TerminalGrid, c: char, color: Color, x:usize, y:usize, len: usize) {
        self.draw_line(c, color, x, y, 1, 0, len);
    }

    pub fn draw_line_vertical(self: &mut TerminalGrid, c: char, color: Color, x:usize, y:usize, len: usize) {
        self.draw_line(c, color, x, y, 0, 1, len);
    }

    pub fn draw_box(self: &mut TerminalGrid, c: char, color: Color, x:usize, y:usize, w:usize, h:usize) {
        for i in x..(x+w).min(self.width) {
            for j in y..(y+h).min(self.height) {
                self.set_cell(c, color, i, j);
            }
        }
    }

    pub fn reset(self: &mut TerminalGrid) {
        self.fill(' ', COLOR_BG);
    }

    pub fn fill(self: &mut TerminalGrid, c: char, color: Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.set_cell(c, color, i,j);
            }
        }
    }

    pub fn display(self: &mut TerminalGrid) {
        let (w,h) = crossterm::terminal::size().unwrap();
        let w = w as usize;
        let h = h as usize;
        if w != self.width || h != self.height {
            self.width = w;
            self.height = h;
            self.grid_size = w*h;
            self.grid.resize(self.grid_size, ColoredChar{c:' ', color: COLOR_BG});
            self.last_grid.resize(self.grid_size, ColoredChar{c: '.', color: COLOR_BG});
        }
        
        // Detect diffs and copy into "last_grid"
        let mut change = false;
        for i in 0..self.grid_size{
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

pub type CC = ColoredChar;

impl ColoredChar {
    pub fn new(character: char, color: Color) -> ColoredChar {
        ColoredChar{c:character, color:color}
    }
    fn to_string(self: &ColoredChar, bg_color: Color) -> String {
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

