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
    pub board: Vec<Vec<Option<Piece>>>,
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
            board: vec![
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
            ],
            logs_rx: logs_tx.subscribe(),
            logs_tx,
            keys_rx: keys_tx.subscribe(),
            keys_thread,
        }
    }

    pub fn log(&mut self, msg: &str) -> anyhow::Result<()> {
        writeln!(self.log_file, "{msg}")?;
        self.log_file.flush()?;
        Ok(())
    }

    pub fn board(&self) -> Table {
        Table::new(vec![
            Row::new(vec!["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"]).height(3),
            Row::new(vec!["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"]).height(3),
            Row::new(vec!["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"]).height(3),
            Row::new(vec!["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"]).height(3),
            Row::new(vec!["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"]).height(3),
            Row::new(vec!["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"]).height(3),
            Row::new(vec!["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"]).height(3),
            Row::new(vec!["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"]).height(3),
        ])
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

/*
pub trait StyledRow<'a> {
    fn apply_style(&'a self) -> Self;
}

impl<'a> StyledRow<'a> for Row<'a> {
    fn apply_style(&'a self) -> Self {
        Row::from(self).height(3)
    }
} */

#[cfg(test)]
mod test_types {
    use crate::game::Piece;
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
