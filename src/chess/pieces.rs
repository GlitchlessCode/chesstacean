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
// pub struct Piece {
//     // x: i16,
//     // y: i16,
//     piece_type: PieceType,
//     color: TeamColor,
//     killed: bool,
// }

// impl Piece {
//     pub fn new(piece: PieceType, color: TeamColor, ) -> Self {
//         Piece {
//             // x: i16,
//             // y: i16,
//             piece_type: piece,
//             color: color,
//             killed: false,
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
//         // if self.check(id) == true
//             match self.piece_type {
//                 King => ,
//                 Queen => ,
//                 Pawn => ,
//                 Bishop => ,
//                 Knight => ,
//                 Rook => ,
//             }
//     }

//     fn check_tile(tile_array: [tiles::Tile<'a>; 64]) -> bool {
//         // check if tile is occupied by own piece
//     }
// }
