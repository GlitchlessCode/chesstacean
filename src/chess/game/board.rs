pub struct Board {
    height: u8,
    width: u8,
    whitecancastle: bool,
    blackcancastle: bool,
}

impl Board {
    pub fn new(height: u8, width: u8) -> Self {
        Board { height: height, width: width, whitecancastle: true, blackcancastle: true}
    }
    
    pub fn new_default() -> Self {
        Board::new(8, 8)
    }
}
