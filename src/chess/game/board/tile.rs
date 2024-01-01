use crate::chess::game::pieces;

pub enum Tile {
    Piece {
        piece: pieces::Piece, // Box<dyn Piece>
                              // [...]
    },
    Empty,
    Wall,
}
