extern crate core;
use core::fmt;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Debug for Color {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ansi = self.ansi();
        write!(f, "Color({:3} {:3} {:3}; {:3}; {:08b})", self.r, self.g, self.b, ansi, ansi)
    }

}

impl Color {

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }

    // ANSI code for this color.
    // See http://en.wikipedia.org/wiki/ANSI_escape_code#Colors
    fn ansi(&self) -> u8 {
        fn c8to3(x: u8) -> u8 {
            (x as usize * 5 / 255) as u8
        }

        16 + 36 * c8to3(self.r) + 6 * c8to3(self.g) + c8to3(self.b)
    }

    // ANSI sequence to set foreground
    pub fn for_fg(&self) -> String {
        format!("\x1B[38;5;{:?}m", self.ansi())
    }

    // ANSI sequence to set background
    pub fn for_bg(&self) -> String {
        format!("\x1B[48;5;{:?}m", self.ansi())
    }

}
