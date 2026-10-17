use interterm::Input;

const CTRLC: u128 = 0x3;

fn main(){
    let mut input = Input::new();
    let stopper = input.stopper();
    input.add_listener(move |buf| {
        if u128::from_be_bytes(buf.clone()) >> 120 == CTRLC{
            stopper.stop();
        }
        println!("You type:{:#x}",u128::from_be_bytes(*buf) >> 31);
    });
    while !input.is_done() {}
}
