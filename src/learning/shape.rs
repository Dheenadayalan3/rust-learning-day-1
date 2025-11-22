// struct and its implementation
pub struct Rect{
    pub width: u32,
    pub height: u32
}

impl Rect{
    pub fn area(&self)-> u32 {
        self.width * self.height
    }
}

