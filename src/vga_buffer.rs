#[allow(dead_code)] // Stop the compiler for complaining about unused variant of the color enum 
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
#[repr(u8)] // Specify that each value is stored as u8 
pub enum Color{
    Black = 0,
    Blue = 1, 
    Green = 2,
    Cyan = 3, 
    Red = 4, 
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

// Implementing the color code
impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode{
        // Start with                                       = 00000000
        // Background shifter 4 bits to left (F.exp green)  = 00100000
        // OR with foreground color(f.eks cyan)             = 00100011
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}


