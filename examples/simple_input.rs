use interterm::Input;

fn main(){
    let mut input = Input::new();
    let stopper = input.stopper();
    input.add_listener(move |buf| {
        if buf == "CTRL+c"{
            stopper.stop();
        }
        println!("You type:{}",buf.to_string());
    });
    while !input.is_done() {}
}
