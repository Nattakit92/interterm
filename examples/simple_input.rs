use std::io;

use interterm::Input;

fn main() -> io::Result<()>{
    let mut input = Input::new();
    let stopper = input.stopper();
    _ = input.raw_mode();
    input.listener(move |event| {
        if event == "CTRL+C"{
            stopper.stop();
        }
        println!("You type:{}",event.to_string());
    });
    input.wait()
}
