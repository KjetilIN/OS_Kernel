use volatile::Volatile;         // Using the volatile create to make the buffer volatile 
use core::fmt;                  // Using fmt for implementing the write macro 
use lazy_static::lazy_static;   // Macro for initiating the static at compile time, when it is first used.


// Defining constants for the size of the screen 
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


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

/// Implementing the color byte code for the VGA buffer
impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode{
        // Start with                                       = 00000000
        // Background shifter 4 bits to left (F.exp green)  = 00100000
        // OR with foreground color(f.eks cyan)             = 00100011
        ColorCode(((background as u8) << 4) | (foreground as u8))
    }
}

/// Struct that represents the a screen character in the VGA buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar{
    ascii_character: u8,
    color_code: ColorCode, 
}

/// The VGA buffer itself with a set size 
#[repr(transparent)]
struct Buffer{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH];BUFFER_HEIGHT],
}

/// Writer that will always write the last line
pub struct Writer{
    column_position: usize, 
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}


impl Writer {
    /// Function for writing a ASCII byte to the buffer
    pub fn write_byte(&mut self, byte:u8){
        match byte {
            // If the byte is a new line
            b'\n' => self.new_line(),

            // Handling the byte value
            byte => {
                // If the column position is on the last position, we go to a new line
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1; 
                let col = self.column_position;

                let color_code = self.color_code;

                // Adding the char to the buffer by using the volatile write method 
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code
                });

                // Incrementing the position 
                self.column_position += 1; 
            }

            
        }
    }


    /// Function for implementing the 
    fn new_line(&mut self){ 
        for row in 1..BUFFER_HEIGHT {
            for col in 1..BUFFER_WIDTH {
                // Reading the character and writing it one row above 
                let character: ScreenChar = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character)

            }
        }

        self.clear_row(BUFFER_HEIGHT - 1); 
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize){
        // Defining a blank ASCII char
        let blank = ScreenChar{
            ascii_character: b' ',
            color_code: self.color_code
        };

        // Write blank ASCII char for the row
        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }

    /// Function that writes a string to the buffer
    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
    
}

/// Implementing the write trait for the VGA buffer writer 
impl fmt::Write for Writer{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// The static writer for the vga buffer module 
    pub static WRITER: Writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe{ &mut *(0xb8000 as *mut Buffer)}
    };
}


/// Function that prints the basic information of the OS
/// Uses the writer struct to write to the VGA buffer
pub fn print_introduction(){
    // Using the write trait for this function
    use core::fmt::Write; 

    // Create a new writer 
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'[');
    writer.write_string("INFO");
    writer.write_byte(b']');

    // Using the macro for writing to VGA Buffer 
    write!(writer, " OS by Kjetil Indrehus").unwrap();
}

