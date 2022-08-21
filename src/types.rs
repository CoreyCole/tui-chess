use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};  

#[derive(Eq, Ord)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn, 
}

impl Piece {
    pub fn display(&self) -> &str {
        match *self {
            Piece::King => "K",
            Piece::Queen => "Q",
            Piece::Rook => "R",
            Piece::Bishop => "B",
            Piece::Knight => "N",
            Piece::Pawn => "P",
        }
    }

    pub fn points(&self) -> u8 {
        match *self {
            Piece::King => 0,
            Piece::Queen => 9,
            Piece::Rook => 5,
            Piece::Bishop => 3,
            Piece::Knight => 3,
            Piece::Pawn => 1,
        }
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.points() == other.points()
    }
}

impl PartialOrd for Piece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.points().cmp(&other.points()))
    }
}


