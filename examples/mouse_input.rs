use std::io::{self, Write};

use interterm::Input;

fn main() -> io::Result<()>{
    let mut input = Input::new();
    print!("\x1b[?1002h\x1b[?1006h");
    io::stdout().flush()?;
    input.raw_mode()?;
    let stopper = input.stopper();
    input.listener(move |buf| {
        let buf_raw = buf.get_raw();
        if buf_raw == "\u{3}"{
            stopper.stop();
        }
        println!("You type:{:?}",buf_raw);
    });
    input.wait();
    print!("\x1b[?1002l\x1b[?1006l");
    Ok(())
}
