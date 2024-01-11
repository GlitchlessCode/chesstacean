use crate::chess::game::pieces::{self, Position};

pub enum Tile {
    Piece {
        piece: pieces::Piece, // Box<dyn Piece>
                              // [...]
    },
    Empty {
        position: Position
    },
    Wall,
}
