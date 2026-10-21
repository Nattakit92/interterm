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

const SHIFT: u8 = 0b0001;
const ALT:   u8 = 0b0010;
const CTRL:  u8 = 0b0100;
const SUPER: u8 = 0b1000;

#[derive(PartialEq)]
enum Key{
    Letter(char), Function(u8),
    Up, Down, Left, Right,
    Enter, Backspace, Tabs, Escape,
    Home, Insert, Delete, End, PgUp, PgDown,
    Undefined
}

impl Key {
    fn from(s: &str) -> Self{

        let mut s_chars = s.chars();
        if let (Some(c), None) = (s_chars.next(),s_chars.next()){
            return Key::Letter(c);
        }
        if let Some(x) = s.strip_prefix('f').or_else(|| s.strip_prefix('F')){
            if let Ok(i) = x.parse::<u8>() && i <= 20{
                return Key::Function(i);
            }
        }

        match s.to_lowercase().as_str() {
            "up" => Key::Up,
            "down" => Key::Down,
            "left" => Key::Left,
            "right" => Key::Right,
            "enter" => Key::Enter,
            "return" => Key::Enter,
            "backspace" => Key::Backspace,
            "tabs" => Key::Tabs,
            "escape" => Key::Escape,
            "home" => Key::Home,
            "insert" => Key::Insert,
            "delete" => Key::Delete,
            "del" => Key::Delete,
            "end" => Key::End,
            "pgup" => Key::PgUp,
            "pageup" => Key::PgUp,
            "pgdown" => Key::PgDown,
            "pagedown" => Key::PgDown,
            _ => Key::Undefined
        }
    }
}

impl ToString for Key{
    fn to_string(&self) -> String {
        use Key::*;
        match self {
            &Letter(c) => c.to_string(),
            &Function(x) => format!("F{}",x),
            &Up => "Up".to_string(),
            &Down => "Down".to_string(),
            &Left => "Left".to_string(),
            &Right => "Right".to_string(),
            &Enter => "Enter".to_string(),
            &Backspace => "Backspace".to_string(),
            &Tabs => "Tabs".to_string(),
            &Escape => "Escape".to_string(),
            &Home => "Home".to_string(),
            &Insert => "Insert".to_string(),
            &Delete => "Delete".to_string(),
            &End => "End".to_string(),
            &PgUp => "PgUp".to_string(),
            &PgDown => "PgDown".to_string(),
            &Undefined => "Unknown".to_string(),
        }
    }
}

pub struct KeyPressed{
    raw: String,
    modifier: u8,
    key: Key
}

impl PartialEq<str> for KeyPressed{
    fn eq(&self, other: &str) -> bool{
        let mut modifier = 0;
        let mut key = Key::Undefined;
        for k in other.split('+').map(|s| s.trim()){
            match k.to_uppercase().as_str() {
                "CTRL" | "CMD" => modifier |= CTRL,
                "SHIFT" => modifier |= SHIFT,
                "ALT" | "OPTION" => modifier |= ALT,
                "SUPER" | "META" => modifier |= SUPER,
                x => key = Key::from(x),
            }
        }
        self.modifier == modifier && self.key == key
    }
}

impl<'a> ToString for KeyPressed{
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
        if self.modifier & SUPER != 0{
            result += "SUPER+"
        }
        result += &self.key.to_string();
        result
    }
}

impl KeyPressed{
    fn undefined(raw: String) -> Self{
        Self { raw, modifier: 0, key: Key::Undefined }
    }
    pub fn from(raw_arr: [u8; 16], n: usize) -> Self{
        let raw;
        match str::from_utf8(&raw_arr[..n]) {
            Ok(x) => raw = x.to_string(),
            Err(_) => return Self::undefined(String::new())
        }
        if !str_has_control(&raw){
            if n == 1{
                let key = Key::Letter(raw.chars().next().unwrap());
                return KeyPressed { raw, modifier: 0, key };
            }
            return Self::undefined(raw);
        }
        let mut modifier = 0;
        let key;
        let mut i = 0;

        match raw_arr[0] {
            //escape code
            b'\x1b' => {
                i = 1;
                if n==1{
                    return KeyPressed { raw, modifier:0, key: Key::Escape };
                }
                modifier |= ALT;
            },
            _ => ()
        }

        match raw_arr[i] {
            b'\n' => key = Key::Enter,
            b'\t' => key = Key::Tabs,

            // CSI sequence
            b'[' if modifier & ALT != 0 => {
                modifier &= !ALT;
                i = 2;
                let csi: Vec<&[u8]> = raw_arr[i..n].split(|x| *x==b';').collect();
                match csi[csi.len()-1] {
                    // vt sequence
                    x if x[x.len()-1] == b'~' => {
                        // let key_code;
                        todo!()
                    }
                    // xterm sequence
                    x => match x[x.len()-1] {
                        b'A' => key = Key::Up,
                        b'B' => key = Key::Down,
                        b'C' => key = Key::Right,
                        b'D' => key = Key::Left,
                        b'P' => key = Key::Function(1),
                        b'Q' => key = Key::Function(2),
                        b'R' => key = Key::Function(3),
                        b'S' => key = Key::Function(4),
                        _ => return Self::undefined(raw)
                    }
                }
            }

            // xterm sequence
            b'O' if i+1!=n => {
                if modifier & ALT == 0{
                    return Self::undefined(raw);
                }
                modifier &= !ALT;
                i = 2;
                match raw_arr[i] {
                    b'P' => key = Key::Function(1),
                    b'Q' => key = Key::Function(2),
                    b'R' => key = Key::Function(3),
                    b'S' => key = Key::Function(4),
                    _ => return Self::undefined(raw)
                }
            }

            0x8 => {
                modifier |= CTRL;
                key = Key::Backspace;
            }
            0x7f => key = Key::Backspace,

            1..=26 => {
                modifier |= CTRL;
                key = Key::Letter((raw_arr[i]+64) as char);
            },
            x if x.is_ascii() => key = Key::Letter(x as char),

            _ => return Self::undefined(raw)
        }
        Self { raw, modifier, key }
    }

    pub fn get_raw(&self) -> &str{
        &self.raw
    }
}

fn str_has_control(s: &str) -> bool{
    s.chars().any(|c| c.is_control())
}
