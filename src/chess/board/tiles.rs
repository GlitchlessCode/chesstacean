
use crate::chess::pieces;
pub enum TileState<'a> {
    Occupied(&'a pieces::Piece),
    Empty
}

pub struct Tile<'a> {
    // x: i16,
    // y: i16,
    id: i16,
    pub state: TileState<'a>
}
