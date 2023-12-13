

mod pieces {
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
        color: TeamColor,
        coordinates: (i16, i16),
        piece_type: PieceType,
    }

    impl Piece {
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
