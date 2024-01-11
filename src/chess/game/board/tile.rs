use crate::chess::game::pieces;

pub enum Tile {
    Piece { piece: pieces::Piece },
    Empty,
    Wall,
}
