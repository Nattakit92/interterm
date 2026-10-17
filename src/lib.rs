use std::io::{self, stdin};
use std::os::fd::AsRawFd;

pub struct Input{
}

impl Input{
    pub fn new() -> io::Result<()> {
        Termios::raw_mode()
    }
}

#[repr(C)]
#[derive(Default,Copy,Clone)]
struct Termios{
    c_iflag: u32,
    c_oflag: u32,
    c_cflag: u32,
    c_lflag: u32,
    c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32
}

impl Termios{
    fn raw_mode() -> io::Result<()>{
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
}

const ECHO: u32 = 0o10;
const ICANON: u32 = 0o2;
const ISIG: u32 = 0o1;
const TCSANOW: i32 = 0;

unsafe extern "C"{
    fn tcgetattr(fd: i32, termios_p: *mut Termios)-> i32;
    fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios)-> i32;
}
