use crate::input_handler::Key::Letter;

pub struct Buffer{
    data: Vec<String>,
    history: Vec<String>,
    x: usize,
    y: usize,
}

impl Buffer{
    pub fn new() -> Self{
        Buffer { data: vec![String::new()], history: Vec::new(), x: 0, y: 0}
    }
    pub fn insert(&mut self, ch: char){
        self.data[self.y].insert(self.x, ch);
        self.x += 1;
    }
}

impl From<Buffer> for String {
    fn from(value: Buffer) -> Self {
        value.data.join("\n")
    }
}

impl From<String> for Buffer{
    fn from(value: String) -> Self {
        let mut data: Vec<String> = vec![String::new()];
        let mut i = 0;
        for c in value.chars(){
            if c == '\n'{
                data.push(String::new());
                i+=1;
            }else {
                data[i].push(c);
            }
        }
        Buffer { data: data.clone(), history: Vec::new(), x: data[i].len()-1, y: i }
    }
}

const CTRL: u8 = 0b1;
const SHIFT: u8 = 0b10;
const ALT: u8 = 0b100;

#[derive(PartialEq)]
enum Key{
    Letter(char),
    Up, Down, Left, Right,
    Enter, Backspace, Delete, Tabs,
    Undefined
}

impl Key {
    fn from(s: &str) -> Self{

        let mut s_chars = s.chars();
        if let (Some(c), None) = (s_chars.next(),s_chars.next()){
            return Letter(c);
        }

        match s.to_lowercase().as_str() {
            "up" => Key::Up,
            "down" => Key::Down,
            "left" => Key::Left,
            "right" => Key::Right,
            "enter" => Key::Enter,
            "backspace" => Key::Backspace,
            "delete" => Key::Delete,
            "tabs" => Key::Tabs,
            _ => Key::Undefined
        }
    }
}

impl ToString for Key{
    fn to_string(&self) -> String {
        use Key::*;
        match self {
            &Letter(c) => c.to_string(),
            &Up => "Up".to_string(),
            &Down => "Down".to_string(),
            &Left => "Left".to_string(),
            &Right => "Right".to_string(),
            &Enter => "Enter".to_string(),
            &Backspace => "Backspace".to_string(),
            &Delete => "Delete".to_string(),
            &Tabs => "Tabs".to_string(),
            &Undefined => "Unknown".to_string(),
        }
    }
}

pub struct KeyPressed{
    raw: [u8; 16],
    modifier: u8,
    key: Key
}

impl PartialEq<str> for KeyPressed{
    fn eq(&self, other: &str) -> bool{
        let mut modifier = 0;
        let mut key = Key::Undefined;
        for k in other.split('+').map(|s| s.trim()){
            match k {
                "CTRL" => modifier |= CTRL,
                "SHIFT" => modifier |= SHIFT,
                "ALT" => modifier |= ALT,
                x => key = Key::from(x),
            }
        }
        self.modifier == modifier && self.key == key
    }
}

impl ToString for KeyPressed{
    fn to_string(&self) -> String {
        let mut result = String::new();
        if self.modifier & CTRL != 0{
            result += "CTRL+"
        }
        if self.modifier & SHIFT != 0{
            result += "SHIFT+"
        }
        if self.modifier & ALT != 0{
            result += "ALT+"
        }
        result += &self.key.to_string();
        result
    }
}

impl KeyPressed{
    pub fn from(raw: [u8; 16], n: usize) -> Self{
        let mut modifier = 0;
        let mut key = Key::Undefined;
        match n {
            1 if raw[0] < 32 => {
                modifier = CTRL;
                key = Key::Letter((raw[0]+96) as char);
            },
            1 if raw[0] == 128 => {
                key = Key::Backspace;
            },
            x if x<=4 => {
                let s = std::str::from_utf8(&raw).unwrap();
                key = Key::Letter(char::from(s.chars().next().unwrap()))
            },
            _ => ()
        }
        KeyPressed{raw, modifier, key}
    }
    pub fn get_raw(&self) -> [u8; 16]{
        self.raw
    }
}
