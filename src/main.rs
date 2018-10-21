#![feature(nll)]
#![feature(core_intrinsics)]
#![feature(duration_as_u128)]
#![feature(test)]

extern crate rand;
extern crate test;

mod bank;
mod bitboard;
mod board;
mod mcts;
mod piece;
mod pieces;
mod player;
mod player_set;
mod shape;
mod placement;

use crate::bitboard::*;
use crate::board::*;
use crate::mcts::*;
use crate::player::*;
use crate::player_set::*;

use std::time::SystemTime;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use crate::shape::*;

fn main() {
    /*
    for piece in pieces::iter() {
        print!("Piece{{orientations:&[");
        for orientation in piece.orientations {
            let mut bits: u128 = 0;
            bits |= (orientation.bits[0] as u128);
            bits |= (orientation.bits[1] as u128) << 60;
            print!(
                "Shape {{bits:0x{:032X},attachments:&{:?},width:{},height:{}}},",
                bits, orientation.attachments, orientation.width, orientation.height
            );
        }
        print!("]}},")
    }*/

    let mut board = Board::new();
    let mut rng = SmallRng::from_entropy();
    let mut turn = Player::Blue;

    let mut out = PlayerSet::new();

    loop {
        if out.is_full() {
            break;
        }

        if out.contains(turn) {
            turn = turn.next();
            continue;
        }

        if board.find_moves(turn).is_empty() {
            out = out.add(turn);
        }

        /*if turn != Player::Blue {
			board.play_randomly(turn, &mut rng);
			board.display();
			turn = turn.next();
			println!();
			continue;
		}*/

        /*board.display();
        println!();*/

        let mut node = Node::new(board.clone(), turn, out.clone());

        if node.is_terminal() {
            break;
        }
        let start = SystemTime::now();

        let mut count = 0;
        while SystemTime::now().duration_since(start).unwrap().as_secs() < 5 {
            for _ in 0..100 {
                node.step(&mut rng);
            }

            count += 100;
        }

        if node.is_terminal() {
            break;
        }

        match turn {
            Player::Blue => print!("Blue"),
            Player::Yellow => print!("Yellow"),
            Player::Red => print!("Red"),
            Player::Green => print!("Green"),
        };

        println!(" ({})", count);

        board = node.best_child(&mut rng).board.clone();
        board.display();
        println!();
        // out = node.out.clone();
        turn = turn.next();

        count += 1;
    }

    println!("{:?}", board.find_wins());
}
