use std::rc::Weak;
use std::sync::Arc;

pub struct Piece {
    piece_type: PieceType,
    color: TeamColor,
}

impl Piece {
    pub fn new(piece: PieceType, color: TeamColor, ) -> Self {
        Piece {
            piece_type: piece,
            color: color,
        }
    }

    pub fn get_value(self) -> i16 {
        match self.piece_type {
            King => 100,
            Queen => 9,
            Pawn => 1,
            Bishop => 3,
            Knight => 3,
            Rook => 5,
        }
    }

    pub fn can_move(self, id: i16) -> bool {
        // check if tile is one of the possible moves in PieceType.get_moves 
        false
    }
}

enum PieceType {
    King,
    Queen,
    Pawn,
    Bishop,
    Knight,
    Rook,
}

impl PieceType {
    pub fn get_moveset (self) {
        match self {
            King => ,
            Queen => ,
            Pawn => ,
            Bishop => ,
            Knight => ,
            Rook => ,
        }
    }
    
    pub fn get_moves(self) -> Vec<Vec<MoveList>> {
        
    }

    pub fn get_valid_moves(self) {
       let moves = self.get_moves();

    }
}

struct MoveList {
    head: Arc<Option<MoveLink>>,
    tail: Arc<Option<MoveLink>>
}

struct MoveLink {
    pos: Position,
    next: Arc<Option<MoveLink>>,
    prev: Weak<Option<MoveLink>>,
}
   
enum Progress<T> {
    Valid(T),
    Invalid(T)
}

enum TeamColor {
    White,
    Black
}

struct Position {
    x: u8,
    y: u8
}

