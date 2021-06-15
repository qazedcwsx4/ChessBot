use colored::{Colorize, ColoredString};

pub enum Color {
    White,
    Black,
}

pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn represent(&self) -> ColoredString {
        let piece = match self.piece_type {
            PieceType::Pawn => "P",
            PieceType::Knight => "K",
            PieceType::Bishop => "B",
            PieceType::Rook => "R",
            PieceType::Queen => "Q",
            PieceType::King => "X",
        };

        match self.color {
            Color::White => piece.red(),
            Color::Black => piece.blue(),
        }
    }
}