use crate::board::STARTER_BOARD;
use crate::controls::InputMode;
use crossterm::event::{self, Event, KeyCode};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fs::File;
use std::io::Write;
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Row, Table},
};

pub struct GameState {
    pub log_file: File,
    pub input_mode: InputMode,
    pub input: String,
    pub messages: Vec<String>,
    pub board: [[Piece; 8]; 8],
    pub logs_tx: Sender<String>,
    pub logs_rx: Receiver<String>,
    pub keys_rx: Receiver<KeyCode>,
    pub keys_thread: JoinHandle<()>,
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

    pub fn piece(x: u8, y: u8) -> Option<Piece> {
        println!("{}, {}", x, y);
        None
    }

    pub fn valid_moves(x: u8, y: u8) -> Option<Vec<(u8, u8)>> {
        println!("{}, {}", x, y);
        None
    }
}

#[derive(Eq)]
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

impl Piece {
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
