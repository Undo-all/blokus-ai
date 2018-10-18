use piece::*;
use pieces;
use std::intrinsics;
use std::iter;

#[derive(Clone)]
pub struct Bank {
    pub pieces: u32,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            pieces: 0x00000000001FFFFF,
        }
    }

    pub fn take(&mut self, piece: usize) -> Self {
        let mut copy = self.clone();
        copy.pieces &= !(1 << piece);
        copy
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
            self.remaining &= !(1 << piece);
            let copy = self.pieces & !(1 << piece);
            Some((pieces::by_id(piece as PieceId), Bank { pieces: copy }))
        }
    }
}
