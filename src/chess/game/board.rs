use super::pieces;

pub struct Board {
    pub tiles: Vec<tile::Tile>,
    pub height: u8,
    pub width: u8,
    pub whitecancastle: bool,
    pub blackcancastle: bool,
}

impl Board {
    pub fn new(fen: String, height: u8, width: u8) -> Self {
        // create starting formation here (10x12 board)
        let mut tile_arr = Vec::new();
        for i in 0..width * 2 + 3 {
            tile_arr.push(tile::Tile::Wall)
        }

        for char in fen.chars() {
            match char.to_digit(10) {
                Some(num) => {
                    for i in 0..num {
                        tile_arr.push(tile::Tile::Empty);
                    }
                }
                None => {
                    if char == '/' {
                        for i in 0..2 {
                            tile_arr.push(tile::Tile::Wall)
                        }
                    } else {
                        tile_arr.push(tile::Tile::Piece { piece: Board::char_to_piece(char) })
                    }
                }
            }
        }

        Board {
            tiles: tile_arr,
            height: height,
            width: width,
            whitecancastle: true,
            blackcancastle: true,
        }
    }

    pub fn new_default() -> Self {
        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/"); // slightly modified fen string used to have dynamic starting positions for custom boards
        Board::new(fen, 8, 8)
    }

    pub fn get_tile(&self, x: u8, y: u8) -> tile::Tile {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
          return tile::Tile::Wall
        }
        self.tiles[(y as u16 * self.width as u16 + x as u16) as usize]
      }

    pub fn char_to_piece(char: char) -> pieces::Piece {
        let color = pieces::TeamColor::Black;
        if char.is_uppercase() {
            color = pieces::TeamColor::White;
        }

        match char.to_ascii_lowercase() {
            'k' => pieces::Piece::King {
                color: color,
                position: /*Get Position*/,
            },
            'q' => pieces::Piece::Queen {
                color: color,
                position: /*Get Position*/,
            },
            'p' => pieces::Piece::Pawn {
                color: color,
                position: /*Get Position*/,
            },
            'b' => pieces::Piece::Bishop {
                color: color,
                position: /*Get Position*/,
            },
            'k' => pieces::Piece::Knight {
                color: color,
                position: /*Get Position*/,
            },
            'r' => pieces::Piece::Rook {
                color: color,
                position: /*Get Position*/,
            },
        }
    }
}
mod tile;
