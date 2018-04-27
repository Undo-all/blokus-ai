#![feature(nll)]
#![feature(core_intrinsics)]

extern crate rand;

mod bitboard;
mod piece;
mod pieces;
mod player;
mod shape;
mod bank;
mod board;
mod mcts;

use player::*;
use board::*;
use mcts::*;

use rand::{SmallRng, FromEntropy};

fn main() {
    let mut board = Board::new();
    let mut rng = SmallRng::from_entropy();
    let mut turn = Player::Orange;

    loop {
        board.display();
        println!();

        let mut node = Node::new(board, turn, false);

        for _ in 0..10000 {
            node.step(&mut rng);
        }

        if node.is_terminal() {
            break;
        }

        board = node.best_child().board.clone();
        turn = turn.opponent();
    }
}
