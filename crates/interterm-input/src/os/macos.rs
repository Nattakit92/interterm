#[repr(C)]
#[derive(Default,Copy,Clone,PartialEq,Eq,Debug)]
pub struct Termios{
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_cc: [u8; 20],
    c_ispeed: u32,
    c_ospeed: u32
}

pub const ECHO: u32 = 0x8;
pub const ICANON: u32 = 0x100;
pub const ISIG: u32 = 0x80;
pub const TCSANOW: i32 = 0;
pub const VTIME: usize = 16;
pub const VMIN: usize = 17;
