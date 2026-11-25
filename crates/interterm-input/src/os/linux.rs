#[repr(C)]
#[derive(Default,Copy,Clone,PartialEq,Eq,Debug)]
pub struct Termios{
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    c_line: u8,
    pub c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32
}

pub const ECHO: u32 = 0o10;
pub const ICANON: u32 = 0o2;
pub const ISIG: u32 = 0o1;
pub const TCSANOW: i32 = 0;
pub const VTIME: usize = 5;
pub const VMIN: usize = 6;
