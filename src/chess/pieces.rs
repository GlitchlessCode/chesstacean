// enum TeamColor {
//     White,
//     Black
// }

// enum PieceType {
//     King,
//     Queen,
//     Pawn,
//     Bishop,
//     Knight,
//     Rook,
// }

// impl PieceType {
//     pub fn get_moves(self) -> Vec<Move> {
//         match self {
//                     King => ,
//                     Queen => ,
//                     Pawn => ,
//                     Bishop => ,
//                     Knight => ,
//                     Rook => ,
//         }
//     }
// }

// enum Move {
//     End, 
//     Pos {
//         next: Box<Progress<Move>>,
//         pos: Position
//     }
// }

// enum Progress<T> {
//     Valid(T),
//     Invalid(T)
// }

// struct Position {
//     x: u8,
//     y: u8
// }

// pub struct Piece {
//     piece_type: PieceType,
//     color: TeamColor,
// }

// impl Piece {
//     pub fn new(piece: PieceType, color: TeamColor, ) -> Self {
//         Piece {
//             piece_type: piece,
//             color: color,
//         }
//     }

//     pub fn get_value(self) -> i16 {
//         match self.piece_type {
//             King => 100,
//             Queen => 9,
//             Pawn => 1,
//             Bishop => 3,
//             Knight => 3,
//             Rook => 5,
//         }
//     }

//     pub fn can_move(self, id: i16) -> bool {
//         // check if tile is one of the possible moves in PieceType.get_moves 
//     }
// }
