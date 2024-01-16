use std::collections::BTreeMap;

use super::pieces::{self, Move, Position, ValidMove};

pub struct Board {
    pub tiles: BTreeMap<Position, tile::Tile>,
    pub height: u8,
    pub width: u8,
    pub black_can_castle: bool,
    pub white_can_castle: bool,
}

impl Board {
    pub fn make_move(&mut self, valid_move: pieces::ValidMove) {}

    pub fn validate_move(&self, try_move: Move) -> Option<ValidMove> {
        Some(try_move.into())
    }

    pub fn new(fen: String, height: u8, width: u8) -> Self {
        // create starting formation here (10x12 board)
        let mut tiles = BTreeMap::new();
        let mut x = 1;
        let mut y = 1;

        for char in fen.chars() {
            match char.to_digit(10) {
                Some(num) => {
                    for i in 0..num {
                        tiles.insert(Position { x, y }, tile::Tile::Empty);
                        x += 1
                    }
                }
                None => {
                    if char == '/' {
                        y += 1;
                        x = 0;
                    } else {
                        let pos = Position { x, y };
                        tiles.insert(
                            pos.clone(),
                            tile::Tile::Piece {
                                piece: Board::char_to_piece(char, pos),
                            },
                        );
                        x += 1;
                    }
                }
            }
        }

        Self {
            tiles,
            height,
            width,
            black_can_castle: true,
            white_can_castle: true,
        }
    }

    pub fn get_tile(&self, pos: &Position) -> &tile::Tile {
        if pos.x >= self.width as u16 || pos.y >= self.height as u16 {
            &tile::Tile::Wall
        } else {
            &self.tiles[pos]
        }
    }

    pub fn char_to_piece(char: char, pos: Position) -> pieces::Piece {
        let color = if char.is_uppercase() {
            pieces::TeamColor::White
        } else {
            pieces::TeamColor::Black
        };

        pieces::Piece::new(
            color,
            pos,
            match char.to_ascii_lowercase() {
                'k' => pieces::PieceType::King,
                'q' => pieces::PieceType::Queen,
                'p' => pieces::PieceType::Pawn,
                'b' => pieces::PieceType::Bishop,
                'n' => pieces::PieceType::Knight,
                'r' => pieces::PieceType::Rook,
                _ => panic!("Should never be this"),
            },
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"); // slightly modified fen string used to have dynamic starting positions for custom boards
        Board::new(fen, 8, 8)
    }
}

mod tile;
