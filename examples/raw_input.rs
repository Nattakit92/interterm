use interterm::Input;

const CTRLC: u128 = 0x3;

fn main(){
    let mut input = Input::new();
    let stopper = input.stopper();
    input.add_listener(move |buf| {
        let buf_raw = buf.get_raw();
        if u128::from_be_bytes(buf_raw) >> 120 == CTRLC{
            stopper.stop();
        }
        println!("You type:{:?}",buf_raw);
    });
    while !input.is_done() {}
}
