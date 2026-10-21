use interterm::Input;

fn main(){
    let mut input = Input::new();
    let stopper = input.stopper();
    _ = input.raw_mode();
    input.listener(move |buf| {
        if buf == "CTRL+C"{
            stopper.stop();
        }
        println!("You type:{}",buf.to_string());
    });
    input.wait();
}
