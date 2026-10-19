use std::io::{self, Read};
use interterm::os::Termios;

fn main() -> io::Result<()>{
    let mut original = Termios::new()?;
    Termios::timeout_decis(10, &mut original.clone())?;
    let mut buffer = [0u8; 32];

    match io::stdin().read(&mut buffer) {
        Ok(n) if n > 0 => {
            println!("\nReceived {} byte(s): {:?}", n, &buffer[..n]);
        }
        Ok(_) => {
            println!("\nTimeout reached! No input received.");
        }
        Err(e) => {
            eprintln!("\nError reading input: {}", e);
        }
    }
    Termios::setattr(&mut original)?;
    Ok(())
}
