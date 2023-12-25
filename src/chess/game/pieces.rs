use std::rc::Weak;
use std::sync::Arc;

pub enum Piece {
    King {
        color: TeamColor,
        position: Position,
    },
    Queen{
        color: TeamColor,
        position: Position,
    },
    Pawn{
        color: TeamColor,
        position: Position,
    },
    Bishop{
        color: TeamColor,
        position: Position,
    },
    Knight{
        color: TeamColor,
        position: Position,
    },
    Rook{
        color: TeamColor,
        position: Position,
    },
}

impl Piece {
    // get color name
    pub fn get_color(self) -> TeamColor {
        match self {
            Piece::King {color:color,position:_} => color,
            Piece::Queen {color:color,position:_}  => color,
            Piece::Pawn {color:color,position:_}  => color,
            Piece::Bishop {color:color,position:_}  => color,
            Piece::Knight {color:color,position:_}  => color,
            Piece::Rook {color:color,position:_}  => color,
        }
    }

    // get position
    pub fn get_position(self) -> Position {
        match self {
            Piece::King {color:_,position:position} => position,
            Piece::Queen {color:_,position:position}  => position,
            Piece::Pawn {color:_,position:position}  => position,
            Piece::Bishop {color:_,position:position}  => position,
            Piece::Knight {color:_,position:position}  => position,
            Piece::Rook {color:_,position:position}  => position,
        }
    }

    // get material value
    pub fn get_value(self) -> i16 {
        match self {
            Piece::King {color:_,position:_} => 100,
            Piece::Queen {color:_,position:_}  => 9,
            Piece::Pawn {color:_,position:_}  => 1,
            Piece::Bishop {color:_,position:_}  => 3,
            Piece::Knight {color:_,position:_}  => 3,
            Piece::Rook {color:_,position:_}  => 5,
        }
    }

    // get moveset for each piece
    pub fn get_moveset(self) {
        match self {
            Piece::King {color:_,position:_}  => (),
            Piece::Queen {color:_,position:_}  => (),
            Piece::Pawn {color:_,position:_}  => (),
            Piece::Bishop {color:_,position:_}  => (),
            Piece::Knight {color:_,position:_}  => (),
            Piece::Rook {color:_,position:_}  => (),
        }
    }

    // piece type functions
    pub fn is_king(self) -> bool {
        match self {
            Piece::King {color:_,position:_}  => true,
            _ => false
        }
    }

    pub fn is_queen(self) -> bool {
        match self {
            Piece::Queen {color:_,position:_}  => true,
            _ => false
        }
    }

    pub fn is_pawn(self) -> bool {
        match self {
            Piece::Pawn {color:_,position:_}  => true,
            _ => false
        }
    }

    pub fn is_bishop(self) -> bool {
        match self {
            Piece::Bishop {color:_,position:_}  => true,
            _ => false
        }
    }

    pub fn is_knight(self) -> bool {
        match self {
            Piece::Knight {color:_,position:_}  => true,
            _ => false
        }
    }

    pub fn is_rook(self) -> bool {
        match self {
            Piece::Rook {color:_,position:_}  => true,
            _ => false
        }
    }

    // 

    // pub fn get_moves(self) -> Vec<Vec<MoveList>> {

    // }

    pub fn get_valid_moves(self) {
        //    let moves = self.get_moves();
    }
    
    pub fn can_move(self, id: i16) -> bool {
        // check if tile is one of the possible moves in Piece.get_moves
        false
    }
}

struct MoveList {
    head: Arc<Option<MoveLink>>,
    tail: Arc<Option<MoveLink>>,
}

struct MoveLink {
    pos: Position,
    next: Arc<Option<MoveLink>>,
    prev: Weak<Option<MoveLink>>,
}

enum Progress<T> {
    Valid(T),
    Invalid(T),
}

enum TeamColor {
    White,
    Black,
}

struct Position {
    x: u8,
    y: u8,
}
