use std::io::{self, stdin};
use std::os::fd::AsRawFd;
use crate::os::*;

impl Termios{
    pub fn raw_mode() -> io::Result<()>{
        let mut termios = Termios::default();
        let stdin = stdin();
        let fd = stdin.as_raw_fd();

        unsafe {
            if tcgetattr(fd, &mut termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }

        termios.c_lflag &= !(ECHO | ICANON | ISIG);

        unsafe {
            if tcsetattr(fd, TCSANOW, &mut termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }
    pub fn timeout_decis(decis: u8) -> io::Result<()>{
        let mut termios = Termios::default();
        let stdin = stdin();
        let fd = stdin.as_raw_fd();

        unsafe {
            if tcgetattr(fd, &mut termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }

        unsafe {cfmakeraw(&mut termios)}
        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = decis;

        unsafe {
            if tcsetattr(fd, TCSANOW, &mut termios) != 0{
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

unsafe extern "C"{
    fn tcgetattr(fd: i32, termios_p: *mut Termios)-> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios)-> i32;
    fn cfmakeraw(termios_p: *mut Termios);
}
