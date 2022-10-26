use crate::game::Piece;

pub const STARTER_BOARD: [[Piece; 8]; 8] = [
    [
        Piece::BlackRook,
        Piece::BlackKnight,
        Piece::BlackBishop,
        Piece::BlackKing,
        Piece::BlackQueen,
        Piece::BlackBishop,
        Piece::BlackKnight,
        Piece::BlackRook,
    ],
    [
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
        Piece::BlackPawn,
    ],
    [
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
    ],
    [
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
    ],
    [
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
    ],
    [
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
        Piece::Empty,
    ],
    [
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
        Piece::WhitePawn,
    ],
    [
        Piece::WhiteRook,
        Piece::WhiteKnight,
        Piece::WhiteBishop,
        Piece::WhiteKing,
        Piece::WhiteQueen,
        Piece::WhiteBishop,
        Piece::WhiteKnight,
        Piece::WhiteRook,
    ],
];
