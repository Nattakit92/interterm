use std::io;

use interterm::Input;

const CTRLC: u128 = 0x3;

fn main() -> io::Result<()>{
    let mut input = Input::new();
    input.raw_mode()?;
    let stopper = input.stopper();
    print!("\x1b[>2u");
    input.add_listener(move |buf| {
        let buf_raw = buf.get_raw();
        if u128::from_be_bytes(buf_raw) >> 120 == CTRLC{
            stopper.stop();
        }
        println!("You type:{:?}",str::from_utf8(&buf_raw).unwrap());
    });
    while !input.is_done() {}
    Ok(())
}
