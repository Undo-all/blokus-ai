#![feature(nll)]
#![feature(core_intrinsics)]
#![feature(duration_as_u128)]
#![feature(test)]

extern crate rand;
extern crate test;

mod bitboard;
mod piece;
mod pieces;
mod player;
mod shape;
mod bank;
mod board;
mod mcts;
mod player_set;

use player::*;
use board::*;
use mcts::*;
use player_set::*;
use bitboard::*;

use std::time::SystemTime;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use shape::*;

fn main() {
	let mut test = Vec::new();
	for piece in pieces::iter() {
		test.push(piece);
	}

	let size = |s: &Shape| { (s.bits[0].count_ones() + s.bits[1].count_ones()) as usize };
	test.sort_by(|a, b| size(&a.orientations[0]).cmp(&size(&b.orientations[0])));
	println!("{:#?}", test);
/*
    let mut board = Board::new();
    let mut rng = SmallRng::from_entropy();
    let mut turn = Player::Blue;
	
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

        if node.is_terminal() { break; }
        let start = SystemTime::now();
        
		let mut count = 0;
        while (SystemTime::now().duration_since(start).unwrap().as_secs() < 1) {
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
		}

		println!(" ({})", count);

        board = node.best_child(&mut rng).board.clone();
		board.display();
		println!();
		// out = node.out.clone();
        turn = turn.next();
		
		count += 1;
    }

	println!("{:?}", board.find_wins());
	*/
}
