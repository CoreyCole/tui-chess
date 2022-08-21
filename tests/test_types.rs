#[cfg(test)]
mod test_types {
    use tui_chess::types::{Piece};
    #[test]
    fn test_display() {
        assert_eq!(Piece::King.display(), "K");
        assert_eq!(Piece::Queen.display(), "Q");
        assert_eq!(Piece::Rook.display(), "R");
        assert_eq!(Piece::Bishop.display(), "B");
        assert_eq!(Piece::Knight.display(), "N");
        assert_eq!(Piece::Pawn.display(), "P");
    }

    #[test]
    fn test_points() {
        assert_eq!(Piece::King.points(), 0);
        assert_eq!(Piece::Queen.points(), 9);
        assert_eq!(Piece::Rook.points(), 5);
        assert_eq!(Piece::Bishop.points(), 3);
        assert_eq!(Piece::Knight.points(), 3);
        assert_eq!(Piece::Pawn.points(), 1);
    }

    #[test]
    fn test_cmp() {
        assert!(Piece::King == Piece::King);
        assert!(Piece::Queen > Piece::Pawn);
    }
}

