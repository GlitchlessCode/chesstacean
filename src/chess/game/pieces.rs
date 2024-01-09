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
    pub fn get_moveset(self) -> FullMoveset {
        match self {
            Piece::King { .. } => FullMoveset{moves: vec![Moveset{x_modifier: -1, y_modifier: 1}, Moveset{x_modifier: 0, y_modifier: 1}, Moveset{x_modifier: 1, y_modifier: 1}, Moveset{x_modifier: -1, y_modifier: 0}, Moveset{x_modifier: 1, y_modifier: 0}, Moveset{x_modifier: -1, y_modifier: -1}, Moveset{x_modifier: 0, y_modifier: -1}, Moveset{x_modifier: 1, y_modifier: -1}] , iterative: false},
            Piece::Queen { .. } => FullMoveset{moves: vec![Moveset{x_modifier: -1, y_modifier: 1}, Moveset{x_modifier: 0, y_modifier: 1}, Moveset{x_modifier: 1, y_modifier: 1}, Moveset{x_modifier: -1, y_modifier: 0}, Moveset{x_modifier: 1, y_modifier: 0}, Moveset{x_modifier: -1, y_modifier: -1}, Moveset{x_modifier: 0, y_modifier: -1}, Moveset{x_modifier: 1, y_modifier: -1}] , iterative: true},
            Piece::Pawn { .. } => FullMoveset{moves: vec![Moveset{x_modifier: 0, y_modifier: -1}], iterative: false},
            Piece::Bishop { .. } => FullMoveset{moves: vec![Moveset{x_modifier: -1, y_modifier: -1}, Moveset{x_modifier: 1, y_modifier: 1}, Moveset{x_modifier: 1, y_modifier: -1}, Moveset{x_modifier: -1, y_modifier: 1}] , iterative: true},
            Piece::Knight { .. } => FullMoveset{moves: vec![Moveset{x_modifier: 2, y_modifier: 1}, Moveset{x_modifier: 1, y_modifier: 2}, Moveset{x_modifier: -1, y_modifier: 2}, Moveset{x_modifier: -2, y_modifier: 1}, Moveset{x_modifier: -2, y_modifier: -1}, Moveset{x_modifier: -1, y_modifier: -2}, Moveset{x_modifier: 1, y_modifier: -2}, Moveset{x_modifier: 2, y_modifier: -1}] , iterative: false},
            Piece::Rook { .. } => FullMoveset{moves: vec![Moveset{x_modifier: -1, y_modifier: 0}, Moveset{x_modifier: 1, y_modifier: 0}, Moveset{x_modifier: 0, y_modifier: -1}, Moveset{x_modifier: 0, y_modifier: 1}], iterative: true},
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

    // pub fn get_all_moves(self) -> Vec<Vec<MoveList>> {
    //     // get movelists for every piece
    // }

    pub fn get_piece_moves(self) -> Vec<MoveList> {
        // get movelists for specific piece
        let moveset = self.get_moveset();
        let all_moves: Vec<MoveList> = Vec::new();
        let piece_position: Position = self.get_position();

        for moveset_list in moveset.moves {
            let new_position = 

            if moveset.iterative == false {
                break;
            };
        }
        all_moves
    }

    pub fn get_valid_moves(self) {
        // validate all piece moves
        //    let moves = self.get_all_moves();
    }

    

    pub fn can_move(self, id: i16) -> bool {
        // check if tile is one of the possible moves in Piece.get_valid_moves
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

struct FullMoveset {
    moves: Vec<Moveset>,
    iterative: bool,
}

struct Moveset {
    x_modifier: i16,
    y_modifier: i16
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
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn add_moveset_to_position(self, moveset: Moveset) -> Position {
        // don't know why this can't find function???
        Position {x: self.add_u16_to_i16(self.x, moveset.x_modifier, y), y: self.add_u16_to_i16(self.x, moveset.y_modifier)}
    }

    pub fn add_i16_to_u16(u16: u16, i16: i16) -> u16 {
        (u16 as i16 + i16 as i16) as u16
    }
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
