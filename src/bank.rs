use piece::*;
use pieces;
use std::iter;
use std::intrinsics;

#[derive(Clone)]
pub struct Bank {
    pieces: u32,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            pieces: 0x00000000001FFFFF,
        }
    }

    pub fn take_iter(&self) -> TakeIter {
        TakeIter {
            pieces: self.pieces,
            remaining: self.pieces,
        }
    }
}

pub struct TakeIter {
    pieces: u32,
    remaining: u32,
}

impl iter::Iterator for TakeIter {
    type Item = (&'static Piece, Bank);

    fn next(&mut self) -> Option<(&'static Piece, Bank)> {
        if self.remaining == 0 {
            None
        } else {
            let piece = unsafe { intrinsics::cttz(self.remaining) as usize };
            self.remaining = self.remaining & !(1 << piece);
            let copy = self.pieces & !(1 << piece);
            Some((pieces::by_id(piece as PieceId), Bank { pieces: copy }))
        }
    }
}
