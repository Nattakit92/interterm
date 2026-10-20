pub mod termios;
pub mod os;
mod input_handler;

use std::io::{stdin,stdout,Read,Write,self};
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

pub struct Input<F: Listener>{
    value: Option<Buffer>,
    listener: Arc<Mutex<Option<F>>>,
    stopper: Arc<Stopper>,
    termios: io::Result<Termios>,
}

impl<F: Listener> Drop for Input<F>{
    fn drop(&mut self){
        if let Ok(original) = self.termios{
            _ = Termios::setattr(&mut original.clone());
        }
    }
}

impl<F: Listener> Input<F>{
    pub fn new() -> Self {
        let original = Termios::new();
        if let Ok(termios) = original{
            _ = Termios::raw_mode(&mut termios.clone());
        }

        let listener: Arc<Mutex<Option<F>>> = Arc::new(Mutex::new(None));
        let thread_listener = Arc::clone(&listener);

        let stopper = Arc::new(Stopper::new());
        let thread_stopper = Arc::clone(&stopper);

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

        Input{value: None, listener: listener, stopper, termios: original}
    }

    pub fn add_listener(&mut self, listener: F){
        *self.listener.lock().unwrap() = Some(listener);
    }

    pub fn stopper(&self) -> Arc<Stopper>{
        Arc::clone(&self.stopper)
    }

    pub fn is_done(&self) -> bool{
        self.stopper.is_stopped()
    }
}
