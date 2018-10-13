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
mod player_set;

use player::*;
use board::*;
use mcts::*;
use player_set::*;

use bitboard::*;

use std::time::SystemTime;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use pieces::*;

fn main() {
    let mut board = Board::new();
    let mut rng = SmallRng::from_entropy();
    let mut turn = Player::Blue;
	
	/*
	for piece in pieces::PIECES.iter() {
		print!("Piece {{\n    orientations: &[\n");
		for shape in piece.orientations {
			let mut new_attach: Vec<u8> = Vec::new();
			for attach in shape.attachments.iter() {
				let x = attach % 14;
				let y = attach / 14;
				new_attach.push(y*20 + x);
			}
			
			let mut new_bits: [u64; 2] = [0, 0];
			for i in 0..64 {
				if ((shape.bits >> i) & 1) == 1 {
					let x = i % 14;
					let y = i / 14;
					let shift = y*20 + x;
					let block = shift / 60;
					let index = shift % 60;
					new_bits[block] |= (1 as u64) << index;
				}
			}

			print!("        Shape: {{\n            bits: &[0x{0:016X}, 0x{1:016X}],\n            attachments: &[", new_bits[0], new_bits[1]);
			print!("{0}", new_attach[0]);
			for attach in new_attach.iter().skip(1) {
				print!(", {0}", attach)
			}

			println!("],\n            width: {0}\n        }},", shape.width);
		}

		println!("    ]\n}},");
	}*/

    loop {
        board.display();
        println!();

        let mut node = Node::new(board, turn, PlayerSet::new());
        if node.is_terminal() { break; }
        let start = SystemTime::now();
        
        let mut playouts = 0;

        while (SystemTime::now().duration_since(start).unwrap().as_secs() < 5) {
            for _ in 0..100 {
                node.step(&mut rng);
            }

            playouts += 100;
        }

        println!("{}", playouts);

        if node.is_terminal() {
            break;
        }

        board = node.best_child().board.clone();
        turn = turn.next();
    }
}
