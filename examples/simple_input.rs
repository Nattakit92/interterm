use interterm::Input;
use std::io::{stdin,Read};

const CTRLC: u128 = 0x3;

fn main(){
    let _ = Input::new();
    loop{
        let mut buf = [0u8; 16];
        let n = stdin().read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        if u128::from_be_bytes(buf) >> ((16 - n) * 8) == CTRLC{
            return;
        }
        println!("You type:{}",str::from_utf8(&buf).unwrap());
    }
}
