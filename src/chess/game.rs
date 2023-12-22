struct Game {
    black: player::Player,
    white: player::Player,
    board: board::Board,
    pieces: Vec<pieces::Piece>,
    status: GameStatus
}

pub enum Action {
    MakeMove,
    OfferDraw,
    AcceptDraw,
}

pub enum GameStatus {
    Ongoing,
    BlackResigned,
    BlackCheckmated,
    WhiteResigned,
    WhiteCheckmated,
    Stalemate,
    DrawAccepted,
}

pub mod board; 

pub mod pieces;

pub mod player;