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

use bitboard::*;
use board::*;
use mcts::*;
use player::*;
use player_set::*;

use std::time::SystemTime;

use rand::{rngs::SmallRng, FromEntropy, Rng};

use shape::*;

fn main() {
    /*
    let mut board = BitBoard::new();
    let enemies = [BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new()];
    board.blocks[3] = (1 as u64) << 30;
    let illegal = board.illegal(Player::Blue, &enemies);

    board.display();
    println!();
    illegal.display();
    println!();

    let mut oriented = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let corners = board.corners(&illegal);
    let mut counter = 0;


    for (i, corner) in corners.iter().enumerate() {
        for (piece_id, piece) in pieces::iter().enumerate() {
            for (orientation_id, orientation) in piece.orientations.iter().enumerate() {
                for (attachment_id, attachment) in orientation.attachments.iter().enumerate() {
                    if board.place_shape(orientation, attachment, corner, &illegal).is_some() {
                        let placement = Placement {
                            piece: piece_id as u8,
                            orientation: orientation_id as u8,
                            attachment: attachment_id as u8,
                        };

                        oriented[i].push(placement);
                    }
                }
            }
        }
    }

    println!("pub const PLACEMENTS: [[Placement; 127]; 4] = {:#?}", oriented);
    */

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
