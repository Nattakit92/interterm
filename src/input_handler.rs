

pub type RawInputBuffer = Vec<u8>;
pub type Modifier = u8;

pub trait BitflagsExt {
    fn contains(&self, other: Self) -> bool;
    fn is_empty(&self) -> bool;
}

impl BitflagsExt for Modifier {
    fn contains(&self, other: Self) -> bool {
        self & other == other
    }
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

const SHIFT:   Modifier = 0b0000001;
const ALT:     Modifier = 0b0000010;
const CTRL:    Modifier = 0b0000100;
const SUPER:   Modifier = 0b0001000;

#[derive(PartialEq,Clone)]
pub enum KeyEvent{
    Letter(char), Function(u8),
    Up, Down, Left, Right,
    Enter, Backspace, Tabs, Escape, Space,
    Home, Insert, Delete, End, PgUp, PgDown,
    Undefined
}

impl From<&str> for KeyEvent {
    fn from(value: &str) -> Self{

        let mut s_chars = value.chars();
        if let (Some(c), None) = (s_chars.next(),s_chars.next()){
            return KeyEvent::Letter(c);
        }
        if let Some(x) = value.strip_prefix('f').or_else(|| value.strip_prefix('F')){
            if let Ok(i) = x.parse::<u8>() && i <= 20{
                return KeyEvent::Function(i);
            }
        }

        match value.to_lowercase().as_str() {
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
            "space" | "spacebar" => KeyEvent::Space,
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
            &Space => "Space".to_string(),
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

#[derive(PartialEq,Clone)]
pub enum MouseEvent {
    LeftClick, RightClick, MiddleClick,
    ScrollUp, ScrollDown, Move,
    Undefined
}

#[derive(PartialEq,Clone)]
pub enum MouseAction {
    Press, Release, Drag, Undefined
}

impl From<&str> for MouseEvent {
    fn from(value: &str) -> Self {
        match value.to_lowercase().replace([' ', '-'], "").as_str() {
            "mouseleft" | "click" | "leftclick" | "lmb" | "primary" | "mouse1" => MouseEvent::LeftClick,
            "mouseright" | "rightclick" | "rmb" | "secondary" | "mouse2" => MouseEvent::RightClick,
            "mousemiddle" | "middleclick" | "mmb" | "scrollwheel" | "scrollclick" | "mouse3" => MouseEvent::MiddleClick,
            "scrollup" | "wheelup" => MouseEvent::ScrollUp,
            "scrolldown" | "wheeldown" => MouseEvent::ScrollDown,
            "move" | "mov" => MouseEvent::Move,
            _ => MouseEvent::Undefined,
        }
    }
}

impl ToString for MouseEvent{
    fn to_string(&self) -> String {
        use MouseEvent::*;
        match self {
            LeftClick => "LMB".to_string(),
            RightClick => "RMB".to_string(),
            MiddleClick => "MMB".to_string(),
            Move => "Move".to_string(),
            ScrollUp => "Scroll-Up".to_string(),
            ScrollDown => "Scroll-Down".to_string(),
            Undefined => String::new(),
        }
    }
}

impl From<&str> for MouseAction{
    fn from(value: &str) -> Self{
        match value.to_lowercase().replace([' ', '-'], "").as_str() {
            "Press" => MouseAction::Press,
            "Release" => MouseAction::Release,
            "Drag" => MouseAction::Drag,
            _ => MouseAction::Undefined
        }
    }
}

impl ToString for MouseAction{
    fn to_string(&self) -> String {
        use MouseAction::*;
        match self {
            Press => "Press".to_string(),
            Release => "Release".to_string(),
            Drag => "Drag".to_string(),
            Undefined => String::new(),
        }
    }
}

#[derive(PartialEq)]
enum EventTypes{
    Keyboard(KeyEvent),
    Mouse(MouseEvent, MouseAction),
    None
}

impl ToString for EventTypes{
    fn to_string(&self) -> String {
        match self {
            Self::None => "Unknown".to_string(),
            Self::Keyboard(k) => k.to_string(),
            Self::Mouse(event,action) => {
                let (event_str, action_str) = (event.to_string(), action.to_string());
                match (!event_str.is_empty(), !action_str.is_empty()) {
                    (true, true) => format!("{event_str}-{action_str}"),
                    (true, false) => event_str.to_string(),
                    (false, true) => action_str.to_string(),
                    (false, false) => String::new(),
                }
            },
        }
    }
}

impl From<KeyEvent> for EventTypes{
    fn from(value: KeyEvent) -> Self {
        Self::Keyboard(value)
    }
}

impl From<(MouseEvent,MouseAction)> for EventTypes{
    fn from(value: (MouseEvent,MouseAction)) -> Self {
        Self::Mouse(value.0,value.1)
    }
}

impl From<&str> for EventTypes{
    fn from(value: &str) -> Self {
        let key = KeyEvent::from(value);
        let mouse = MouseEvent::from(value);

        if !matches!(key,KeyEvent::Undefined){
            return EventTypes::from(key);
        }

        if !matches!(mouse,MouseEvent::Undefined){
            return EventTypes::from((mouse,MouseAction::Press));
        }
        EventTypes::None
    }
}

pub struct Event{
    raw: String,
    modifier: Modifier,
    event: EventTypes,
    mouse_pos: Option<(u16,u16)>
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
        self.modifier == modifier && self.event == EventTypes::Keyboard(key)
    }
}

impl ToString for Event{
    fn to_string(&self) -> String {
        let mut result = String::new();
        if self.modifier.contains(CTRL){
            result += "CTRL+"
        }
        if self.modifier.contains(SHIFT){
            result += "SHIFT+"
        }
        if self.modifier.contains(ALT){
            result += "ALT+"
        }
        if self.modifier.contains(SUPER){
            result += "SUPER+"
        }
        if !self.modifier.is_empty() && matches!(self.event,EventTypes::Keyboard(_)){
            result += &self.event.to_string().to_uppercase();
        }else{
            result += &self.event.to_string();
        }
        result
    }
}

impl Event{
    pub fn new() -> Self{
        Self{ raw: String::new(), modifier: 0, event: EventTypes::None, mouse_pos: None}
    }

    pub fn update(&mut self, raw_arr: RawInputBuffer, n: usize){
        // reset modifier and key pressed
        self.raw = String::new();
        self.modifier = 0;
        self.event = EventTypes::None;

        match str::from_utf8(&raw_arr[..n]) {
            Ok(x) => self.raw = x.to_string(),
            Err(_) => {
                return;
            }
        }
        if !str_has_control(&self.raw){
            self.event = EventTypes::from(KeyEvent::Letter(self.raw.chars().next().unwrap()));
            return;
        }
        let mut i = 0;

        match raw_arr[0] {
            //escape code
            b'\x1b' => {
                i = 1;
                if n==1{
                    self.event = EventTypes::from(KeyEvent::Escape);
                    return;
                }
                self.modifier |= ALT;
            },
            _ => ()
        }

        match raw_arr[i] {
            b' ' => self.event = EventTypes::from(KeyEvent::Space),
            b'\n' | b'\r' => self.event = EventTypes::from(KeyEvent::Enter),
            b'\t' => self.event = EventTypes::from(KeyEvent::Tabs),

            // CSI sequence
            b'[' if self.modifier & ALT != 0 => {
                i = 2;
                let csi: Vec<&[u8]> = raw_arr[i..n-1].split(|x| *x==b';').collect();
                let key_code: u8;
                key_code = arr_to_u8(csi[0]);
                if let Some(x) = csi.get(1){
                    self.modifier = arr_to_u8(*x) - 1;
                }else{
                    self.modifier = 0;
                }
                self.event = match raw_arr[n-1] {
                    // vt sequence
                    x if x == b'~' => {
                        match key_code {
                            1 | 7 => EventTypes::from(KeyEvent::Home),
                            2 => EventTypes::from(KeyEvent::Insert),
                            3 => EventTypes::from(KeyEvent::Delete),
                            4 | 8 => EventTypes::from(KeyEvent::End),
                            5 => EventTypes::from(KeyEvent::PgUp),
                            6 => EventTypes::from(KeyEvent::PgDown),
                            27 => {
                                match arr_to_u8(csi[2]) {
                                    13 => EventTypes::from(KeyEvent::Enter),
                                    _ => return,
                                }
                            },
                            x if (10..=15).contains(&x) => EventTypes::from(KeyEvent::Function(x-10)),
                            x if (17..=21).contains(&x) => EventTypes::from(KeyEvent::Function(x-11)),
                            x if (23..=26).contains(&x) => EventTypes::from(KeyEvent::Function(x-12)),
                            x if (28..=29).contains(&x) => EventTypes::from(KeyEvent::Function(x-13)),
                            x if (31..=34).contains(&x) => EventTypes::from(KeyEvent::Function(x-14)),
                            _ => return,
                        }
                    }
                    // xterm sequence
                    b'A' => EventTypes::from(KeyEvent::Up),
                    b'B' => EventTypes::from(KeyEvent::Down),
                    b'C' => EventTypes::from(KeyEvent::Right),
                    b'D' => EventTypes::from(KeyEvent::Left),
                    b'F' => EventTypes::from(KeyEvent::End),
                    b'H' => EventTypes::from(KeyEvent::Home),
                    b'P' => EventTypes::from(KeyEvent::Function(1)),
                    b'Q' => EventTypes::from(KeyEvent::Function(2)),
                    b'R' => EventTypes::from(KeyEvent::Function(3)),
                    b'S' => EventTypes::from(KeyEvent::Function(4)),
                    b'u' => {
                        let c = arr_to_u8(csi[csi.len()-2]) as char;
                        if !c.is_ascii_alphabetic(){
                            return;
                        }
                        EventTypes::from(KeyEvent::Letter(c))
                    }
                    // mouse input
                    x if x == b'M' || x == b'm' => {
                        let mut default_mouse_action = match x {
                            b'M' => MouseAction::Press,
                            _ => MouseAction::Release,
                        };
                        self.mouse_pos = Some((arr_to_u16(csi[1]), arr_to_u16(csi[2])));
                        let first_csi = arr_to_u8(&csi[0][1..]);
                        self.modifier = match first_csi {
                            x if x.contains(16) => CTRL,
                            x if x.contains(8) => ALT,
                            x if x.contains(4) => SHIFT,
                            _ => 0,
                        };
                        let mouse_event = match first_csi {
                            x if x.contains(65) => {
                                default_mouse_action = MouseAction::Undefined;
                                MouseEvent::ScrollDown
                            },
                            x if x.contains(64) => {
                                default_mouse_action = MouseAction::Undefined;
                                MouseEvent::ScrollUp
                            },
                            x if x.contains(3) => MouseEvent::Move,
                            x if x.contains(2) => MouseEvent::RightClick,
                            x if x.contains(1) => MouseEvent::MiddleClick,
                            x if x.contains(0) => MouseEvent::LeftClick,
                            _ => MouseEvent::Undefined
                        };
                        let mouse_action = match first_csi {
                            x if x.contains(32) => MouseAction::Drag,
                            _ => default_mouse_action
                        };
                        EventTypes::from((mouse_event,mouse_action))
                    },
                    _ => return
                };
            }

            // xterm sequence
            b'O' if i+1 != n => {
                if self.modifier & ALT == 0{
                    return;
                }
                self.modifier &= !ALT;
                i = 2;
                self.event = match raw_arr[i] {
                    b'P' => EventTypes::from(KeyEvent::Function(1)),
                    b'Q' => EventTypes::from(KeyEvent::Function(2)),
                    b'R' => EventTypes::from(KeyEvent::Function(3)),
                    b'S' => EventTypes::from(KeyEvent::Function(4)),
                    _ => return
                };
            }

            0x8 => {
                self.modifier |= CTRL;
                self.event = EventTypes::from(KeyEvent::Backspace);
            }
            0x7f => self.event = EventTypes::from(KeyEvent::Backspace),

            1..=26 => {
                self.modifier |= CTRL;
                self.event = EventTypes::from(KeyEvent::Letter((raw_arr[i]+64) as char));
            },
            x if x.is_ascii() => self.event = EventTypes::from(KeyEvent::Letter(x as char)),

            _ => return
        }
    }

    pub fn get_raw(&self) -> &str{
        &self.raw
    }

    pub fn get_mouse_pos(&self) -> Option<(u16,u16)>{
        self.mouse_pos
    }

    pub fn get_mouse_input(&self) -> Option<(MouseEvent, MouseAction)>{
        match &self.event {
            EventTypes::Mouse(event, action) => {
                Some((event.clone(),action.clone()))
            }
            _ => None,
        }
    }

    pub fn get_key_input(&self) -> Option<KeyEvent>{
        match &self.event {
            EventTypes::Keyboard(key) => {
                Some(key.clone())
            }
            _ => None,
        }
    }

    pub fn get_key_mod(&self) -> Modifier{
        self.modifier.clone()
    }
}

fn arr_to_u8(arr: &[u8]) -> u8{
    let mut result: u8 = 0;
    for byte in arr{
        if !byte.is_ascii_digit(){
            return result;
        }
        result *= 10;
        result += byte - b'0';
    }
    result
}

fn arr_to_u16(arr: &[u8]) -> u16{
    let mut result: u16 = 0;
    for byte in arr{
        if !byte.is_ascii_digit(){
            return result;
        }
        result *= 10;
        result += (byte - b'0') as u16;
    }
    result
}

fn str_has_control(s: &str) -> bool{
    s.chars().any(|c| c.is_control() || c == ' ')
}
