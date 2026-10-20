use interterm::Input;

fn main(){
    let mut input = Input::new();
    let stopper = input.stopper();
    input.raw_mode();
    input.add_listener(move |buf| {
        if buf == "CTRL+C"{
            stopper.stop();
        }
        println!("You type:{}",buf.to_string());
    });
    while !input.is_done() {}
}
