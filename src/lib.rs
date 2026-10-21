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

pub trait ListenerFn : Fn(&KeyPressed) + Send + Sync + 'static{}
impl<T: Fn(&KeyPressed) + Send + Sync + 'static> ListenerFn for T {}

pub type Listener = Option<Box<dyn ListenerFn>>;

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

struct TermiosHandler{
    termios: Termios,
    original: Termios
}

impl Drop for TermiosHandler{
    fn drop(&mut self){
        _ = Termios::setattr(&mut self.original)
    }
}

impl TermiosHandler {
    fn new() -> Option<Self>{
        match Termios::new() {
            Ok(termios) => {
                Some(TermiosHandler {
                    termios: termios.clone(),
                    original: termios
                })
            },
            Err(_) => None
        }
    }

    fn set_read_timeout(&mut self, decisec: u8) -> io::Result<()>{
        Termios::timeout_decis(decisec, &mut self.termios)
    }

    fn raw_mode(&mut self) -> io::Result<()>{
        Termios::raw_mode(&mut self.termios)
    }
}

pub struct Input{
    value: Option<Buffer>,
    listener: Arc<Mutex<Listener>>,
    stopper: Arc<Stopper>,
    termios: Option<TermiosHandler>,
}

impl Input{
    pub fn new() -> Self {

        let termios = TermiosHandler::new();

        let listener = Arc::new(Mutex::new(None));
        let stopper = Arc::new(Stopper::new());


        Input{value: None, listener: listener, stopper, termios}
    }

    pub fn raw_mode(&mut self) -> io::Result<()>{
        match self.termios.as_mut() {
            Some(termios) => termios.raw_mode(),
            None => Err(Error::new(io::ErrorKind::Other, "Cannot set to raw mode"))
        }
    }

    pub fn add_listener<F>(&mut self, listener: F)
    where
        F: ListenerFn
    {
        let thread_stopper = Arc::clone(&self.stopper);
        let thread_listener = Arc::clone(&self.listener);

        thread::spawn(move || {
            stdout().flush().unwrap();
            while !thread_stopper.is_stopped(){
                let mut buf = [0u8; 16];
                let n = stdin().read(&mut buf).unwrap();
                if n == 0{
                    continue;
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
    pub fn set_read_timeout(&mut self, decisec: u8) -> io::Result<()>{
        match self.termios.as_mut() {
            Some(termios) => termios.set_read_timeout(decisec),
            None => Err(Error::new(io::ErrorKind::Other, "Cannot set read timeout"))
        }
    }
}
