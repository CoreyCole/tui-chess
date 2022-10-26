use crate::board::STARTER_BOARD;
use crate::controls::InputMode;
use crate::ui::Screen;
use crossterm::event::{self, Event, KeyCode};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fs::File;
use std::io::Write;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tui::{
    layout::Constraint,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Row, Table},
};

pub struct GameState {
    pub screen: Screen,
    pub log_file: File,
    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,
    pub board: [[Piece; 8]; 8],
    pub logs_tx: Sender<String>,
    pub logs_rx: Receiver<String>,
    pub keys_rx: Receiver<KeyCode>,
    keys_thread: JoinHandle<()>,
}

impl GameState {
    fn send_keys(keys_tx: Sender<KeyCode>) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        if keys_tx.send(key.code).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("event::read() err: {e}");
                        break;
                    }
                    _ => {}
                }
            }
        })
    }

    pub fn new(log_file: File) -> Self {
        let (logs_tx, _rx) = broadcast::channel(10);
        let (keys_tx, _rx) = broadcast::channel(10);
        let keys_thread = Self::send_keys(keys_tx.clone());
        Self {
            screen: Screen::Board,
            log_file,
            input_mode: InputMode::Normal,
            input: "".into(),
            messages: Vec::new(),
            board: STARTER_BOARD,
            logs_rx: logs_tx.subscribe(),
            logs_tx,
            keys_rx: keys_tx.subscribe(),
            keys_thread,
        }
    }

    pub fn stop(&self) {
        // TODO: exit automatically without key press needed
        self.keys_thread.abort();
    }

    pub fn log(&mut self, msg: &str) -> anyhow::Result<()> {
        writeln!(self.log_file, "{msg}")?;
        self.log_file.flush()?;
        Ok(())
    }

    pub fn board(&self) -> Table {
        let mut rows = Vec::new();
        for i in 0..8 {
            let mut row = Vec::new();
            for j in 0..8 {
                row.push(self.board[i][j].display_string());
            }
            rows.push(Row::new(row).height(6));
        }
        Table::new(rows)
            .style(Style::default().fg(Color::White))
            .header(
                Row::new(vec!["a", "b", "c", "d", "e", "f", "g", "h"])
                    .style(Style::default().fg(Color::Yellow))
                    .bottom_margin(1),
            )
            .block(Block::default())
            .widths(&[
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
                Constraint::Length(10),
            ])
            .column_spacing(1)
    }

    pub fn logs(&self) -> List {
        let list_items: Vec<ListItem> = self
            .messages
            .iter()
            .map(|m| ListItem::new(Span::raw(m.to_string())))
            .rev()
            .collect();
        List::new(list_items).block(Block::default().borders(Borders::ALL).title("Logs"))
    }

    pub fn piece(&self, x: usize, y: usize) -> Option<Piece> {
        match &self.board[x][y] {
            Piece::Empty => None,
            p => Some(*p),
        }
    }

    pub fn moves(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let piece = &self.board[x][y];
        let mut moves = Vec::new();
        let mut valid_moves = Vec::new();
        match piece {
            Piece::WhiteQueen | Piece::BlackQueen => {
                moves.push(Move::Lateral(7));
                moves.push(Move::Diagonal(7));
            }
            Piece::WhiteKing | Piece::BlackKing => {
                moves.push(Move::Lateral(1));
                moves.push(Move::Diagonal(1));
            }
            Piece::WhiteRook | Piece::BlackRook => moves.push(Move::Lateral(7)),
            Piece::WhiteBishop | Piece::BlackBishop => moves.push(Move::Diagonal(7)),
            Piece::WhiteKnight | Piece::BlackKnight => moves.push(Move::Knight),
            Piece::WhitePawn => moves.push(Move::WhitePawn),
            Piece::BlackPawn => moves.push(Move::BlackPawn),
            Piece::Empty => {}
        }
        for m in moves {
            match m {
                Move::Lateral(n) => {
                    // TODO: go from piece out until intersecting another piece
                    // left to right
                    let left = if n > x { 0 } else { x - n };
                    let right = if n + x > 7 { 8 } else { n + x };
                    for i in left..right {
                        if !self.board[i][y].matches_color(piece) {
                            valid_moves.push((i, y));
                        }
                    }
                    // top to bottom
                    let top = if n > y { 0 } else { y - n };
                    let bottom = if n + y > 7 { 8 } else { n + y };
                    for i in top..bottom {
                        if !self.board[x][i].matches_color(piece) {
                            valid_moves.push((x, i));
                        }
                    }
                }
                Move::Diagonal(_n) => {}
                Move::Knight => {}
                Move::WhitePawn => {}
                Move::BlackPawn => {}
            }
        }
        valid_moves
    }
}

#[derive(Copy, Clone, Eq)]
pub enum Piece {
    Empty,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

#[derive(Eq, PartialEq)]
pub enum ChessColor {
    White,
    Black,
}

enum Move {
    Lateral(usize),
    Diagonal(usize),
    Knight,
    WhitePawn,
    BlackPawn,
}

impl Piece {
    pub fn matches_color(&self, other: &Self) -> bool {
        self.color() == other.color()
    }

    pub fn color(&self) -> Option<ChessColor> {
        match *self {
            Piece::Empty => None,
            Piece::WhiteKing
            | Piece::WhiteQueen
            | Piece::WhiteRook
            | Piece::WhiteBishop
            | Piece::WhiteKnight
            | Piece::WhitePawn => Some(ChessColor::White),
            Piece::BlackKing
            | Piece::BlackQueen
            | Piece::BlackRook
            | Piece::BlackBishop
            | Piece::BlackKnight
            | Piece::BlackPawn => Some(ChessColor::Black),
        }
    }

    pub fn display_string(&self) -> String {
        String::from(self.display())
    }

    pub fn display(&self) -> &str {
        match *self {
            Piece::Empty => "",
            Piece::WhiteKing => "W K",
            Piece::WhiteQueen => "W Q",
            Piece::WhiteRook => "W R",
            Piece::WhiteBishop => "W B",
            Piece::WhiteKnight => "W N",
            Piece::WhitePawn => "W P",
            Piece::BlackKing => "B K",
            Piece::BlackQueen => "B Q",
            Piece::BlackRook => "B R",
            Piece::BlackBishop => "B B",
            Piece::BlackKnight => "B N",
            Piece::BlackPawn => "B P",
        }
    }

    pub fn points(&self) -> u8 {
        match *self {
            Piece::WhiteKing | Piece::BlackKing | Piece::Empty => 0,
            Piece::WhiteQueen | Piece::BlackQueen => 9,
            Piece::WhiteRook | Piece::BlackRook => 5,
            Piece::WhiteBishop | Piece::BlackBishop => 3,
            Piece::WhiteKnight | Piece::BlackKnight => 3,
            Piece::WhitePawn | Piece::BlackPawn => 1,
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

#[cfg(test)]
mod test_types {
    use crate::game::Piece;
    #[test]
    fn test_points() {
        assert_eq!(Piece::WhiteKing.points(), 0);
        assert_eq!(Piece::WhiteQueen.points(), 9);
        assert_eq!(Piece::WhiteRook.points(), 5);
        assert_eq!(Piece::WhiteBishop.points(), 3);
        assert_eq!(Piece::WhiteKnight.points(), 3);
        assert_eq!(Piece::WhitePawn.points(), 1);
        assert_eq!(Piece::WhiteKing.points(), Piece::BlackKing.points());
        assert_eq!(Piece::WhiteQueen.points(), Piece::BlackQueen.points());
        assert_eq!(Piece::WhiteRook.points(), Piece::BlackRook.points());
        assert_eq!(Piece::WhiteBishop.points(), Piece::BlackBishop.points());
        assert_eq!(Piece::WhiteKnight.points(), Piece::BlackKnight.points());
        assert_eq!(Piece::WhitePawn.points(), Piece::BlackPawn.points());
    }

    #[test]
    fn test_cmp() {
        assert!(Piece::WhiteKing == Piece::BlackKing);
        assert!(Piece::WhiteQueen > Piece::WhitePawn);
    }
}
