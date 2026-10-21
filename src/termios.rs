use std::io::{self, stdin};
use std::os::fd::AsRawFd;
use crate::os::*;

impl Termios{
    pub fn new() -> io::Result<Self>{
        let mut termios = Termios::default();
        let stdin = stdin();
        let fd = stdin.as_raw_fd();

        unsafe {
            if tcgetattr(fd, &mut termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }

        Ok(termios)
    }
    pub fn raw_mode(termios: &mut Termios) -> io::Result<()>{
        termios.c_lflag &= !(ECHO | ICANON | ISIG);

        Self::setattr(termios)
    }
    pub fn timeout_decis(decis: u8, termios: &mut Termios) -> io::Result<()>{

        termios.c_lflag &= !ICANON;
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = decis;

        Self::setattr(termios)
    }
    pub fn setattr(termios: &mut Termios) -> io::Result<()>{
        let stdin = stdin();
        let fd = stdin.as_raw_fd();

        unsafe {
            if tcsetattr(fd, TCSANOW, termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }

        Ok(())
    }
}

unsafe extern "C"{
    fn tcgetattr(fd: i32, termios_p: *mut Termios)-> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios)-> i32;
}
