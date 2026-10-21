use std::io;
use interterm::Input;

fn main() -> io::Result<()>{

    let mut input = Input::new();
    input.set_read_timeout(10)?;

    let mut s = String::new();

    println!("You have 1 second type something:");

    match io::stdin().read_line(&mut s) {
        Ok(n) if n > 0 => {
            println!("\nReceived {} byte(s): {:?}", n, s);
        }
        Ok(_) => {
            println!("\nTimeout reached! No input received.");
        }
        Err(e) => {
            eprintln!("\nError reading input: {}", e);
        }
    }
    Ok(())
}
