use std::fmt::{Debug, Result, Display};
use serde::__private::Formatter;
use crate::piece::{Piece, PieceType, Color};
use crate::piece::Color::{Black, White};
use crate::piece::PieceType::{Rook, Knight, Bishop, Queen, King, Pawn};

pub struct Board {
    board: [Option<Piece>; 64],
}

const KING_ROW: [PieceType; 8] = [
    Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook,
];

const PAWN_ROW: [PieceType; 8] = [
    Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,
];

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            board: [None; 64]
        };
        board.create_row(KING_ROW, 0, Black);
        board.create_row(PAWN_ROW, 1, Black);

        board.create_row(PAWN_ROW, 6, White);
        board.create_row(KING_ROW, 7, White);
        board
    }

    pub fn move_piece_string(&mut self, game_move: String) {}

    pub fn move_piece(&mut self, source: i32, dest: i32) {}
}

impl Board {
    fn create_row(&mut self, pieces: [PieceType; 8], row: i32, color: Color) {
        let mut i = row * 8;
        for &piece_type in pieces.iter() {
            self.board[i as usize] = Some(Piece { piece_type, color });
            i = i + 1;
        }
    }
}

fn create_white(piece_type: PieceType) -> Option<Piece> {
    create(piece_type, White)
}

fn create_black(piece_type: PieceType) -> Option<Piece> {
    create(piece_type, Black)
}

fn create(piece_type: PieceType, color: Color) -> Option<Piece> {
    Some(Piece { color, piece_type })
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let counter = 0;
        write!(f, "JD")
    }
}