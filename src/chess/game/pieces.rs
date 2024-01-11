use std::rc::Weak;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

pub struct Piece {
    color: TeamColor,
    position: Position,
    kind: PieceType,
    moves: Vec<MoveList>,
}

impl Piece {
    // get color name
    pub fn get_color<'a>(&'a self) -> &'a TeamColor {
        &self.color
    }

    // get position
    pub fn get_position<'a>(&'a self) -> &'a Position {
        &self.position
    }

    // get material value
    pub fn get_value(&self) -> i16 {
        match self.kind {
            PieceType::King => 100,
            PieceType::Queen => 9,
            PieceType::Pawn => 1,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Rook => 5,
        }
    }

    // get moveset for each piece
    pub fn get_moveset(&self) {}

    // piece type functions
    pub fn is_king(&self) -> bool {
        self.kind == PieceType::King
    }

    pub fn is_queen(&self) -> bool {
        self.kind == PieceType::Queen
    }

    pub fn is_pawn(&self) -> bool {
        self.kind == PieceType::Pawn
    }

    pub fn is_bishop(&self) -> bool {
        self.kind == PieceType::Bishop
    }

    pub fn is_knight(&self) -> bool {
        self.kind == PieceType::Knight
    }

    pub fn is_rook(&self) -> bool {
        self.kind == PieceType::Rook
    }

    //

    // pub fn get_moves(&self) -> Vec<Vec<MoveList>> {

    // }

    pub fn get_valid_moves(&self) {
        //    let moves = self.get_moves();
    }

    pub fn can_move(&self, id: i16) -> bool {
        // check if tile is one of the possible moves in Piece.get_moves
        false
    }
}

#[derive(PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Pawn,
    Bishop,
    Knight,
    Rook,
}

impl PieceType {
    pub fn get_valid_moves(&self) {}
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
