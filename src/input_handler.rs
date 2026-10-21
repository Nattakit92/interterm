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
            "enter" | "return" => Key::Enter,
            "backspace" => Key::Backspace,
            "tabs" => Key::Tabs,
            "escape" => Key::Escape,
            "home" => Key::Home,
            "insert" => Key::Insert,
            "del" | "delete" => Key::Delete,
            "end" => Key::End,
            "pgup" | "pageup" => Key::PgUp,
            "pgdown" | "pagedown" => Key::PgDown,
            "space" | "spacebar" => Key::Letter(' '),
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
        if self.modifier != 0{
            result += &self.key.to_string().to_uppercase();
        }else{
            result += &self.key.to_string();
        }
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
            b'\n' | b'\r' => key = Key::Enter,
            b'\t' => key = Key::Tabs,

            // CSI sequence
            b'[' if modifier & ALT != 0 => {
                i = 2;
                let csi: Vec<&[u8]> = raw_arr[i..n].split(|x| *x==b';').collect();
                key = match csi[csi.len()-1] {
                    // vt sequence
                    x if x[x.len()-1] == b'~' => {
                        let key_code: Option<u8>;
                        match csi.len() {
                            1 => key_code = csi.get(0)
                                .and_then(|slice| arr_to_u8(&slice[..=slice.len()-2])),
                            2 => key_code = csi.get(0)
                                .and_then(|slice| arr_to_u8(*slice)),
                            // xterm sequence
                            3 => return Self::undefined(raw),
                            _ => return Self::undefined(raw),
                        }
                        if key_code.is_none(){
                            return Self::undefined(raw);
                        }
                        match key_code.unwrap() {
                            1 | 7 => Key::Home,
                            2 => Key::Insert,
                            3 => Key::Delete,
                            4 | 8 => Key::End,
                            5 => Key::PgUp,
                            6 => Key::PgDown,
                            x if (10..=15).contains(&x) => Key::Function(x-10),
                            x if (17..=21).contains(&x) => Key::Function(x-11),
                            x if (23..=26).contains(&x) => Key::Function(x-12),
                            x if (28..=29).contains(&x) => Key::Function(x-13),
                            x if (31..=34).contains(&x) => Key::Function(x-14),
                            _ => return Self::undefined(raw),
                        }
                    }
                    // xterm sequence
                    x => match x[x.len()-1] {
                        b'A' => Key::Up,
                        b'B' => Key::Down,
                        b'C' => Key::Right,
                        b'D' => Key::Left,
                        b'F' => Key::End,
                        b'H' => Key::Home,
                        b'P' => Key::Function(1),
                        b'Q' => Key::Function(2),
                        b'R' => Key::Function(3),
                        b'S' => Key::Function(4),
                        b'u' => {
                            if let Some(c) = csi.get(csi.len()-2)
                                .and_then(|slice| arr_to_u8(*slice))
                            {
                                Key::Letter(c as char)
                            }
                            else { return Self::undefined(raw); }
                        }
                        _ => return Self::undefined(raw)
                    }
                };
                if csi.len() > 1 && let Some(x) = csi.get(csi.len()-1)
                    .and_then(|slice| arr_to_u8(&slice[..=slice.len()-2]))
                {
                    modifier = x-1;
                } else {
                    modifier = 0;
                }
            }

            // xterm sequence
            b'O' if i+1!=n => {
                if modifier & ALT == 0{
                    return Self::undefined(raw);
                }
                modifier &= !ALT;
                i = 2;
                key = match raw_arr[i] {
                    b'P' => Key::Function(1),
                    b'Q' => Key::Function(2),
                    b'R' => Key::Function(3),
                    b'S' => Key::Function(4),
                    _ => return Self::undefined(raw)
                };
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

fn arr_to_u8(arr: &[u8]) -> Option<u8>{
    let mut result: u8 = 0;
    for byte in arr{
        if !byte.is_ascii_digit(){
            return None;
        }
        result *= 10;
        result += byte - b'0';
    }
    Some(result)
}

fn str_has_control(s: &str) -> bool{
    s.chars().any(|c| c.is_control())
}
