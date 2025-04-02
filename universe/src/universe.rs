

pub struct Universe {
    width: u32,
    height: u32,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
        }
    }

    pub fn get_area(&self) -> u32 {
        return self.width * self.height;
    }
}