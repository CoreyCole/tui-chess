use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};  
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    layout::{
        Constraint,
    },
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
    Terminal,
};

pub struct GameState {
    board: Vec<Vec<Option<Piece>>>,
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

impl GameState {
    pub fn new() -> Self {
        GameState {
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
        }
    }

    pub fn board(&self) -> Table {
        Table::new(vec![
            Row::new(vec!["a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1"]).height(3),
            Row::new(vec!["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"]).height(3),
            Row::new(vec!["a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3"]).height(3),
            Row::new(vec!["a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4"]).height(3),
            Row::new(vec!["a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5"]).height(3),
            Row::new(vec!["a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6"]).height(3),
            Row::new(vec!["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"]).height(3),
            Row::new(vec!["a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"]).height(3),
        ])
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["a", "b", "c", "d", "e", "f", "g", "h"])
            .style(Style::default().fg(Color::Yellow))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1)
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
        return None;
    }

    pub fn valid_moves(x: u8, y: u8) -> Option<Vec<(u8, u8)>> {
        return None;
    }
}

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


