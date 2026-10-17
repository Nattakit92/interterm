mod termios;
use std::io;
use crate::termios::Termios;

pub struct Input{
}

impl Input{
    pub fn new() -> io::Result<()> {
        Termios::raw_mode()
    }
}
