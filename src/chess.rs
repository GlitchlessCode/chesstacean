pub mod board {
    use std::collections::btree_map::Range;


    struct Board {
        tiles: [tile::Tile; 64],
    }

    impl Board {
        fn reset_board(self) {
            for tile in self.tiles {
                tile.state = None
            }

            // add default configuration here later
        }
    }


    pub mod tile {
        enum TileState {
            Occupied(chesstacean::chess::pieces::Piece),
            Empty
        }

        pub struct Tile {
            x: i16,
            y: i16,
            pub state: TileState
        }
    }
}

pub mod pieces {
    enum TeamColor {
        White,
        Black
    }

    enum PieceType {
        King,
        Queen,
        Pawn,
        Bishop,
        Knight,
        Rook,
    }
    pub struct Piece {
        x: i16,
        y: i16,
        color: TeamColor,
        piece_type: PieceType,
        killed: bool,
    }

    impl Piece {
        pub fn new(piece: PieceType) {
            match piece {
                King => Piece::new_king(),
                Queen => Piece::new_piece(9),
                Pawn => Piece::new_piece(1),
                Bishop => Piece::new_piece(3),
                Knight => Piece::new_piece(3),
                Rook => Piece::new_piece(5),
            }
        }

        fn new_king() {

        }

        fn new_piece(value: i16) {

        }

        fn create_object() {
            Piece {
                x: i16,
                y: i16,
                color: TeamColor,
                piece_type: PieceType,
                killed: bool,
            }
        }

        fn get_value(self) -> i16 {
            match self.piece_type {
                King => 100,
                Queen => 9,
                Pawn => 1,
                Bishop => 3,
                Knight => 3,
                Rook => 5,
            }
        }
    }
}
pub mod user;
