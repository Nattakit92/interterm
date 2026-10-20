mod os;
mod termios;
mod input_handler;

use std::io::{self, Error, Read, Write, stdin, stdout};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::{Arc,Mutex};
use std::thread;

use crate::os::Termios;
use crate::input_handler::Buffer;
use crate::input_handler::KeyPressed;

pub trait Listener : Fn(&KeyPressed) + Send + Sync + 'static{}
impl<T: Fn(&KeyPressed) + Send + Sync + 'static> Listener for T {}

pub struct Stopper{
    flag: AtomicBool
}

impl Stopper{
    pub fn new() -> Self{
        Stopper { flag: AtomicBool::new(false) }
    }
    pub fn stop(&self){
        self.flag.store(true, Ordering::SeqCst);
    }
    pub fn is_stopped(&self) -> bool{
        self.flag.load(Ordering::SeqCst)
    }
}

pub struct Input{
    value: Option<Buffer>,
    listener: Arc<Mutex<Option<Box<dyn Listener>>>>,
    stopper: Arc<Stopper>,
    termios: Option<Termios>,
}

impl Drop for Input{
    fn drop(&mut self){
        if let Some(original) = self.termios{
            _ = Termios::setattr(&mut original.clone());
        }
    }
}

impl Input{
    pub fn new() -> Self {
        let original;
        match Termios::new() {
            Ok(termios) => original = Some(termios.clone()),
            Err(_) => original = None
        }

        let listener = Arc::new(Mutex::new(None));
        let stopper = Arc::new(Stopper::new());


        Input{value: None, listener: listener, stopper, termios: original}
    }

    pub fn raw_mode(&self){
        match self.termios {
            Some(termios) => _ = Termios::raw_mode(&mut termios.clone()),
            None => ()
        }
    }

    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: Listener
    {
        let thread_stopper = Arc::clone(&self.stopper);
        let thread_listener = Arc::clone(&self.listener);

        thread::spawn(move || {
            stdout().flush().unwrap();
            while !thread_stopper.is_stopped(){
                let mut buf = [0u8; 16];
                let n = stdin().read(&mut buf).unwrap();
                if n == 0{
                    thread_stopper.stop();
                    break;
                }
                let guard = thread_listener.lock().unwrap();
                let keypressed = KeyPressed::from(buf, n);
                for l in guard.iter(){
                    l(&keypressed);
                }
            }
        });

        *self.listener.lock().unwrap() = Some(Box::new(listener));
    }

    pub fn stopper(&self) -> Arc<Stopper>{
        Arc::clone(&self.stopper)
    }

    pub fn is_done(&self) -> bool{
        self.stopper.is_stopped()
    }

    /// Sets a read timeout for input operations in deciseconds (1/10 of a second).
    pub fn set_read_timeout(&self, decisec: u8) -> io::Result<()>{
        match self.termios {
            Some(termios) => Termios::timeout_decis(decisec, &mut termios.clone()),
            None => Err(Error::new(io::ErrorKind::Other, "Cannot set read timeout"))
        }
    }
}
