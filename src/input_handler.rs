pub type RawInputBuffer = [u8;32];

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
enum KeyEvent{
    Letter(char), Function(u8),
    Up, Down, Left, Right,
    Enter, Backspace, Tabs, Escape,
    Home, Insert, Delete, End, PgUp, PgDown,
    Undefined
}

impl KeyEvent {
    fn from(s: &str) -> Self{

        let mut s_chars = s.chars();
        if let (Some(c), None) = (s_chars.next(),s_chars.next()){
            return KeyEvent::Letter(c);
        }
        if let Some(x) = s.strip_prefix('f').or_else(|| s.strip_prefix('F')){
            if let Ok(i) = x.parse::<u8>() && i <= 20{
                return KeyEvent::Function(i);
            }
        }

        match s.to_lowercase().as_str() {
            "up" => KeyEvent::Up,
            "down" => KeyEvent::Down,
            "left" => KeyEvent::Left,
            "right" => KeyEvent::Right,
            "enter" | "return" => KeyEvent::Enter,
            "backspace" => KeyEvent::Backspace,
            "tabs" => KeyEvent::Tabs,
            "escape" => KeyEvent::Escape,
            "home" => KeyEvent::Home,
            "insert" => KeyEvent::Insert,
            "del" | "delete" => KeyEvent::Delete,
            "end" => KeyEvent::End,
            "pgup" | "pageup" => KeyEvent::PgUp,
            "pgdown" | "pagedown" => KeyEvent::PgDown,
            "space" | "spacebar" => KeyEvent::Letter(' '),
            _ => KeyEvent::Undefined
        }
    }
}

impl ToString for KeyEvent{
    fn to_string(&self) -> String {
        use KeyEvent::*;
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

pub struct Event{
    raw: String,
    modifier: u8,
    key: KeyEvent
}

impl PartialEq<str> for Event{
    fn eq(&self, other: &str) -> bool{
        let mut modifier = 0;
        let mut key = KeyEvent::Undefined;
        for k in other.split('+').map(|s| s.trim()){
            match k.to_uppercase().as_str() {
                "CTRL" | "CMD" => modifier |= CTRL,
                "SHIFT" => modifier |= SHIFT,
                "ALT" | "OPTION" => modifier |= ALT,
                "SUPER" | "META" => modifier |= SUPER,
                x => key = KeyEvent::from(x),
            }
        }
        self.modifier == modifier && self.key == key
    }
}

impl<'a> ToString for Event{
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
        if self.modifier != 0{
            result += &self.key.to_string().to_uppercase();
        }else{
            result += &self.key.to_string();
        }
        result
    }
}

impl Event{
    fn undefined(raw: String) -> Self{
        Self { raw, modifier: 0, key: KeyEvent::Undefined }
    }

    pub fn from(raw_arr: RawInputBuffer, n: usize) -> Self{
        let raw;
        match str::from_utf8(&raw_arr[..n]) {
            Ok(x) => raw = x.to_string(),
            Err(_) => return Self::undefined(String::new())
        }
        if !str_has_control(&raw){
            if n == 1{
                let key = KeyEvent::Letter(raw.chars().next().unwrap());
                return Event { raw, modifier: 0, key };
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
                    return Event { raw, modifier:0, key: KeyEvent::Escape };
                }
                modifier |= ALT;
            },
            _ => ()
        }

        match raw_arr[i] {
            b'\n' | b'\r' => key = KeyEvent::Enter,
            b'\t' => key = KeyEvent::Tabs,

            // CSI sequence
            b'[' if modifier & ALT != 0 => {
                i = 2;
                let csi: Vec<&[u8]> = raw_arr[i..n-1].split(|x| *x==b';').collect();
                let key_code: u8;
                key_code = arr_to_u8(csi[0]);
                if let Some(x) = csi.get(1){
                    modifier = arr_to_u8(*x) - 1;
                }else{
                    modifier = 0;
                }
                key = match raw_arr[n-1] {
                    // vt sequence
                    x if x == b'~' => {
                        match key_code {
                            1 | 7 => KeyEvent::Home,
                            2 => KeyEvent::Insert,
                            3 => KeyEvent::Delete,
                            4 | 8 => KeyEvent::End,
                            5 => KeyEvent::PgUp,
                            6 => KeyEvent::PgDown,
                            27 => {
                                match arr_to_u8(csi[2]) {
                                    13 => KeyEvent::Enter,
                                    _ => return Self::undefined(raw),
                                }
                            },
                            x if (10..=15).contains(&x) => KeyEvent::Function(x-10),
                            x if (17..=21).contains(&x) => KeyEvent::Function(x-11),
                            x if (23..=26).contains(&x) => KeyEvent::Function(x-12),
                            x if (28..=29).contains(&x) => KeyEvent::Function(x-13),
                            x if (31..=34).contains(&x) => KeyEvent::Function(x-14),
                            _ => return Self::undefined(raw),
                        }
                    }
                    // xterm sequence
                    b'A' => KeyEvent::Up,
                    b'B' => KeyEvent::Down,
                    b'C' => KeyEvent::Right,
                    b'D' => KeyEvent::Left,
                    b'F' => KeyEvent::End,
                    b'H' => KeyEvent::Home,
                    b'P' => KeyEvent::Function(1),
                    b'Q' => KeyEvent::Function(2),
                    b'R' => KeyEvent::Function(3),
                    b'S' => KeyEvent::Function(4),
                    b'u' => {
                        let c = arr_to_u8(csi[csi.len()-2]) as char;
                        if !c.is_ascii_alphabetic(){
                            return Self::undefined(raw);
                        }
                        KeyEvent::Letter(c)
                    }
                    _ => return Self::undefined(raw)
                };
            }

            // xterm sequence
            b'O' if i+1 != n => {
                if modifier & ALT == 0{
                    return Self::undefined(raw);
                }
                modifier &= !ALT;
                i = 2;
                key = match raw_arr[i] {
                    b'P' => KeyEvent::Function(1),
                    b'Q' => KeyEvent::Function(2),
                    b'R' => KeyEvent::Function(3),
                    b'S' => KeyEvent::Function(4),
                    _ => return Self::undefined(raw)
                };
            }

            0x8 => {
                modifier |= CTRL;
                key = KeyEvent::Backspace;
            }
            0x7f => key = KeyEvent::Backspace,

            1..=26 => {
                modifier |= CTRL;
                key = KeyEvent::Letter((raw_arr[i]+64) as char);
            },
            x if x.is_ascii() => key = KeyEvent::Letter(x as char),

            _ => return Self::undefined(raw)
        }
        Self { raw, modifier, key }
    }

    pub fn get_raw(&self) -> &str{
        &self.raw
    }
}

fn arr_to_u8(arr: &[u8]) -> u8{
    let mut result: u8 = 0;
    for byte in arr{
        if !byte.is_ascii_digit(){
            return 0;
        }
        result *= 10;
        result += byte - b'0';
    }
    result
}

fn str_has_control(s: &str) -> bool{
    s.chars().any(|c| c.is_control())
}
