struct Game {
    black: player::Player,
    white: player::Player,
    board: board::Board,
    pieces: Vec<pieces::Piece>,
}

pub mod board; 

pub mod pieces;

pub mod player;