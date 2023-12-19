pub struct Board {
    height: u8,
    width: u8,
}

impl Board {
    pub fn new() -> Self {
        Board { height: 8, width: 8 }
    }
}
