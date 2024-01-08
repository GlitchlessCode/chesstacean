use std::rc::Weak;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub enum Piece {
    King { color: TeamColor, position: Position },
    Queen { color: TeamColor, position: Position },
    Pawn { color: TeamColor, position: Position },
    Bishop { color: TeamColor, position: Position },
    Knight { color: TeamColor, position: Position },
    Rook { color: TeamColor, position: Position },
}

impl Piece {
    // get color name
    pub fn get_color(self) -> TeamColor {
        match self {
            Piece::King { color, position: _ } => color,
            Piece::Queen { color, position: _ } => color,
            Piece::Pawn { color, position: _ } => color,
            Piece::Bishop { color, position: _ } => color,
            Piece::Knight { color, position: _ } => color,
            Piece::Rook { color, position: _ } => color,
        }
    }

    // get position
    pub fn get_position(self) -> Position {
        match self {
            Piece::King { color: _, position } => position,
            Piece::Queen { color: _, position } => position,
            Piece::Pawn { color: _, position } => position,
            Piece::Bishop { color: _, position } => position,
            Piece::Knight { color: _, position } => position,
            Piece::Rook { color: _, position } => position,
        }
    }

    // get material value
    pub fn get_value(self) -> i16 {
        match self {
            Piece::King { .. } => 100,
            Piece::Queen { .. } => 9,
            Piece::Pawn { .. } => 1,
            Piece::Bishop { .. } => 3,
            Piece::Knight { .. } => 3,
            Piece::Rook { .. } => 5,
        }
    }

    // get moveset for each piece
    pub fn get_moveset(self) -> Moveset {
        match self {
            Piece::King { .. } => Moveset{moves: vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)] , iterative: false} ,
            Piece::Queen { .. } => Moveset{moves: vec![(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)] , iterative: true},
            Piece::Pawn { .. } => Moveset{moves: vec![(0, -1)], iterative: false},
            Piece::Bishop { .. } => Moveset{moves: vec![(-1, -1), (1, 1), (1, -1), (-1, 1)] , iterative: true},
            Piece::Knight { .. } => Moveset{moves: vec![(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)] , iterative: false},
            Piece::Rook { .. } => Moveset{moves: vec![(-1, 0), (1, 0), (0, -1), (0, 1)], iterative: true},
        }
    }

    // piece type functions
    pub fn is_king(self) -> bool {
        match self {
            Piece::King { .. } => true,
            _ => false,
        }
    }

    pub fn is_queen(self) -> bool {
        match self {
            Piece::Queen { .. } => true,
            _ => false,
        }
    }

    pub fn is_pawn(self) -> bool {
        match self {
            Piece::Pawn { .. } => true,
            _ => false,
        }
    }

    pub fn is_bishop(self) -> bool {
        match self {
            Piece::Bishop { .. } => true,
            _ => false,
        }
    }

    pub fn is_knight(self) -> bool {
        match self {
            Piece::Knight { .. } => true,
            _ => false,
        }
    }

    pub fn is_rook(self) -> bool {
        match self {
            Piece::Rook { .. } => true,
            _ => false,
        }
    }

    pub fn get_all_moves(self) -> Vec<Vec<MoveList>> {
        // get movelists for every piece
    }

    pub fn get_piece_moves(self, moveset: Moveset, ) -> Vec<MoveList> {
        // get movelist for specific piece
        for directions in 0..num_directions {
            
        }
            
    }

    pub fn get_valid_moves(self) {
        // validate all piece moves
        //    let moves = self.get_all_moves();
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

struct Moveset {
    moves: Vec<(i8, i8)>,
    iterative: bool,
}

enum Progress<T> {
    Valid(T),
    Invalid(T),
}

pub enum TeamColor {
    White,
    Black,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Deserialize)]
pub struct Move {
    pub source: Position,
    pub target: Position,
    // More
}

impl From<Move> for ValidMove {
    fn from(value: Move) -> Self {
        let Move { source, target } = value;
        Self { source, target }
    }
}

#[derive(Debug, Serialize)]
pub struct ValidMove {
    source: Position,
    target: Position,
}
