use std::io::{self, Write};

use interterm::{Input,MouseMode};
use interterm::input_handler::{MouseAction, MouseEvent};

fn main() -> io::Result<()>{
    let mut input = Input::new();
    io::stdout().flush()?;
    input.raw_mode()?;
    input.set_mouse_mode(MouseMode::Everything)?;
    let stopper = input.stopper();
    input.add_listener(move |buf| {
        if buf == "CTRL+C"{
            stopper.stop();
        }
    });
    input.add_listener(move |buf| {
        let (x,y) = buf.get_mouse_pos().unwrap();
        if buf.get_mouse_input().is_some(){
            if matches!(buf.get_mouse_input().unwrap().1, MouseAction::Drag){
                println!("pos: {}, {}", x, y);
            }
            if !matches!(buf.get_mouse_input().unwrap().0, MouseEvent::Move){
                println!("action: {}",buf.to_string());
            }
        }
    });
    input.wait()
}
