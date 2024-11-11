use std::io;

use ansi_term::Color::RGB;
use ansi_term::{ANSIByteStrings, ANSIGenericString, Style};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate};

use crate::colors::{Color, BLOCK_CHAR};

const BSU: &[u8] = "\x1B[?2026".as_bytes();
const ESU: &[u8] = "\x1B[?2026l".as_bytes();
const HIDE: &[u8] = "\x1b[?25l".as_bytes();

pub struct TerminalGrid {
    grid: Vec<ColoredChar>,
    last_grid: Vec<ColoredChar>,
    pub width: usize,
    pub height: usize,
    pub grid_size: usize,
    bg_color: Color,
}

impl TerminalGrid {
    pub fn new(bg_color: Color) -> TerminalGrid {
        let (w, h) = crossterm::terminal::size().unwrap();
        let w = w as usize;
        let h = h as usize;
        let grid_size = w * h;
        TerminalGrid {
            grid: vec![
                ColoredChar {
                    c: BLOCK_CHAR,
                    color: bg_color
                };
                grid_size
            ],
            last_grid: vec![
                ColoredChar {
                    c: BLOCK_CHAR,
                    color: (255, 255, 255),
                };
                grid_size
            ],
            width: w,
            height: h,
            grid_size,
            bg_color,
        }
    }

    pub fn index_2d(self: &TerminalGrid, i: usize, j: usize) -> usize {
        j * self.width + i
    }

    pub fn set_cell(self: &mut TerminalGrid, c: char, color: Color, i: usize, j: usize) {
        let t = self.index_2d(i, j);
        if t > self.grid_size {
            println!("sdfsdf");
            return;
        }
        self.grid[t] = ColoredChar { c, color };
    }

    pub fn get_cell(self: &TerminalGrid, i: usize, j: usize) -> ColoredChar {
        self.grid[self.index_2d(i, j)]
    }

    pub fn get_line(self: &TerminalGrid, j: usize) -> String {
        let start_idx = self.index_2d(0, j);
        self.grid[start_idx..start_idx + self.width]
            .iter()
            .map(|cc| cc.to_string(self.bg_color))
            .collect()
    }

    pub fn get_lines(self: &TerminalGrid) -> String {
        let mut result = String::from("");
        for (i, line) in self.grid.chunks(self.width).enumerate() {
            let joined_line: String = line
                .iter()
                .map(|colored_char| colored_char.to_string(self.bg_color))
                .collect();
            result.push_str(&joined_line);

            if i < self.height - 1 {
                result.push('\n');
            }
        }
        result
    }

    pub fn draw_line_h(
        self: &mut TerminalGrid,
        c: char,
        color: Color,
        x: usize,
        y: usize,
        len: i32,
    ) {
        if len < 0 {
            let len = len.abs() as usize;
            self.draw_box(c, color, x - len, y, len, 1);
        } else {
            self.draw_box(c, color, x, y, len as usize, 1);
        }
    }

    pub fn draw_line_v(
        self: &mut TerminalGrid,
        c: char,
        color: Color,
        x: usize,
        y: usize,
        len: i32,
    ) {
        if len < 0 {
            let len = len.abs() as usize;
            self.draw_box(c, color, x, y - len, 1, len);
        } else {
            self.draw_box(c, color, x, y, 1, len as usize);
        }
    }

    pub fn draw_box(
        self: &mut TerminalGrid,
        c: char,
        color: Color,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
    ) {
        for i in x..(x + w).min(self.width) {
            for j in y..(y + h).min(self.height) {
                self.set_cell(c, color, i, j);
            }
        }
    }

    pub fn clear(self: &mut TerminalGrid) {
        self.fill(' ', self.bg_color);
    }

    pub fn fill(self: &mut TerminalGrid, c: char, color: Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.set_cell(c, color, i, j);
            }
        }
    }

    pub fn display(self: &mut TerminalGrid) {
        // Resize char buffer if needed
        let (w, h) = crossterm::terminal::size().unwrap();
        let w = w as usize;
        let h = h as usize;
        if w != self.width || h != self.height {
            self.width = w;
            self.height = h;
            self.grid_size = w * h;
            self.grid.resize(
                self.grid_size,
                ColoredChar {
                    c: ' ',
                    color: self.bg_color,
                },
            );
            self.last_grid = vec![
                ColoredChar {
                    c: '\n',
                    color: (0, 0, 0)
                };
                self.grid.len()
            ];
        }

        // Detect diffs
        let mut diffs: Vec<(usize, usize, ColoredChar)> = vec![];
        for i in 0..self.width {
            for j in 0..self.height {
                let idx = self.index_2d(i, j);
                if self.grid[idx] != self.last_grid[idx] || idx < 12 {
                    self.last_grid[idx] = self.grid[idx];
                    diffs.push((i, j, self.grid[idx]));
                }
            }
        }

        // TODO determine if sync output is supported at runtime
        // Render diffs
        let temp_style = Style::new();

        let draw_commands = diffs.iter().flat_map(|(x, y, cc)| {
            std::iter::once(
                temp_style.paint(format!("\x1B[{};{}H", y + 1, x + 1).as_bytes().to_owned()),
            )
            .chain(std::iter::once(cc.to_ansi(self.bg_color)))
        });

        let supports_synchronized_output = false;
        let draw: Vec<ANSIGenericString<[u8]>> = if supports_synchronized_output {
            std::iter::once(temp_style.paint(BSU))
                .chain(std::iter::once(temp_style.paint(HIDE)))
                .chain(draw_commands)
                .chain(std::iter::once(temp_style.paint(ESU)))
                .collect()
        } else {
            draw_commands
                .chain(std::iter::once(temp_style.paint(HIDE)))
                .collect()
        };

        ANSIByteStrings(&draw)
            .write_to(&mut std::io::stdout())
            .unwrap();

        /*
        execute!(
            io::stdout(),
            BeginSynchronizedUpdate,
            MoveTo(0, 0),
            Print(self.get_lines()),
            Hide,
            EndSynchronizedUpdate
        )
        .unwrap();
        */
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
        self.painted(bg_color).paint(self.c.to_string()).to_string()
    }
    pub fn to_ansi(self: &ColoredChar, bg_color: Color) -> ANSIGenericString<'_, [u8]> {
        self.painted(bg_color)
            .paint(self.c.to_string().as_bytes().to_owned())
    }
    fn painted(self, bg_color: Color) -> Style {
        let color = RGB(self.color.0, self.color.1, self.color.2);
        let bg_color = RGB(bg_color.0, bg_color.1, bg_color.2);
        color.on(bg_color)
    }
}

impl PartialEq for ColoredChar {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.color == other.color
    }
}
