use std::io::{self, Write};

use interterm::{Input,MouseMode};

fn main() -> io::Result<()>{
    let mut input = Input::new();
    io::stdout().flush()?;
    input.raw_mode()?;
    input.set_mouse_mode(MouseMode::Everything)?;
    let stopper = input.stopper();
    input.listener(move |buf| {
        let buf_raw = buf.get_raw();
        if buf_raw == "\u{3}"{
            stopper.stop();
        }
        println!("You type:{:?}",buf_raw);

        // let mouse_pos = buf.get_mouse_pos();
        // if buf == "CTRL+C"{
        //     stopper.stop();
        // }
        // if mouse_pos.is_some(){
        //     println!("Mouse pos: {},{}",mouse_pos.unwrap().0,mouse_pos.unwrap().1)
        // }
    });
    input.wait()
}
