pub struct Buffer{
    data: Vec<String>,
    x: usize,
    y: usize,
}

impl Buffer{
    pub fn new() -> Self{
        Buffer { data: vec![String::new()], x: 0, y: 0 }
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
        Buffer { data: data.clone(), x: data[i].len()-1, y: i }
    }
}
