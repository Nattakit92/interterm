use std::io;

use interterm_input::Input;

fn main() -> io::Result<()>{
    let mut input = Input::new();
    _ = input.raw_mode();
    let stopper = input.stopper();
    input.add_listener(move |event| {
        let buf_raw = event.get_raw();
        if buf_raw == "\u{3}"{
            stopper.stop();
        }
        println!("You type:{:?}",buf_raw);
    });
    input.wait()
}
