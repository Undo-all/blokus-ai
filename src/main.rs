#![feature(nll)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate lazy_static;
extern crate rand;

mod bitboard;
mod piece;
mod pieces;
mod player;
mod shape;

use bitboard::*;
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let mut test = BitBoard::new()
        .place_shape(&pieces::PIECES[0].orientations[0], &0, 0, &BitBoard::new())
        .unwrap();

    let empty = BitBoard::new();
}
