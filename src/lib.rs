//! A very simple display driver for the CU40026 vacuum fluorescent display (VFD).
//!
//! This has only been tested on `CU40026MCPB-S35A` so far.
//!
//!
//! Usage :
//! ```rust
//! let serial_interface = SerialInterface::new(tx);
//! let display = cu40026::Display::new(serial_interface);
//!
//! display.initialise()?;
//! Timer::after(Duration::from_millis(10)).await;
//! display.set_cursor_type(CursorType::InvisibleCursor)?;
//! display.set_luminance(100)?;
//! display.write_str("Hello World!")?;


#![no_std]

pub mod interface;

use interface::Interface;

/// Available display commands
#[repr(u8)]
pub enum Command {
    BackSpace = 0x08,
    HorizontalTab = 0x09,
    LineFeed = 0x0A,
    FormFeed = 0x0C,
    CarriageReturn = 0x0D,

    OverwriteMode = 0x11,
    ScrollUpMode = 0x12,

    UnderlineCursor = 0x14,
    BlinkingSquareCursor = 0x15,
    InvisibleCursor = 0x16,

    Escape = 0x1B,

    MoveCursor = 0x48,
    Luminance = 0x4C,
    BlinkSpeed = 0x54,
    Initialise = 0x49,
}

/// Available cursor types
#[repr(u8)]
pub enum CursorType {
    UnderlineCursor = Command::UnderlineCursor as u8,
    BlinkingSquareCursor = Command::BlinkingSquareCursor as u8,
    InvisibleCursor = Command::InvisibleCursor as u8,
}

/// Display using a generic interface implementation
pub struct Display<I: Interface> {
    interface: I,
}

impl<I: Interface> Display<I> {
    pub fn new(interface: I) -> Self {
        Display { interface }
    }

    /// Write bytes to the display
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), I::Error> {
        self.interface.write(bytes)
    }

    /// Write a string slice to the display
    pub fn write_str(&mut self, str: &str) -> Result<(), I::Error> {
        self.write_bytes(str.as_bytes())
    }

    /// Set the cursor type
    pub fn set_cursor_type(&mut self, cursor_type: CursorType) -> Result<(), I::Error> {
        self.write_bytes(&[cursor_type as u8])
    }

    /// Move the cursor to a given index
    pub fn move_cursor(&mut self, index: u8) -> Result<(), I::Error> {
        self.write_bytes(&[Command::Escape as u8, Command::MoveCursor as u8, index])
    }

    /// Set the luminance of the display
    pub fn set_luminance(&mut self, luminance: u8) -> Result<(), I::Error> {
        self.write_bytes(&[Command::Escape as u8, Command::Luminance as u8, luminance])
    }

    /// Set the cursor blink speed.
    /// The blink period is the period supplied times 30ms
    pub fn set_blink_speed(&mut self, period: u8) -> Result<(), I::Error> {
        self.write_bytes(&[Command::Escape as u8, Command::BlinkSpeed as u8, period])
    }

    /// Initialise the display
    pub fn initialise(&mut self) -> Result<(), I::Error> {
        self.write_bytes(&[Command::Escape as u8, Command::Initialise as u8])
    }

}
