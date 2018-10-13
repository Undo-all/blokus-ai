use bank::*;
use bitboard::*;
use rand::Rng;
use player::Player;
use pieces;

#[derive(Clone)]
pub struct Board {
    placed: [BitBoard; 4],
    banks: [Bank; 4],
}

impl Board {
    pub fn new() -> Self {
        Board {
            placed: [BitBoard::new(), BitBoard::new(), BitBoard::new(), BitBoard::new()],
            banks: [Bank::new(), Bank::new(), Bank::new(), Bank::new()],
        }
    }

    pub fn find_moves(&mut self, player: Player) -> Vec<Board> {
        let board = &self.placed[player as usize];
        
        let (corners, illegal) = if board.is_empty() {
            let start = match player {
                Player::Blue => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 0, &BitBoard::new()).unwrap(),
                Player::Yellow => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 19, &BitBoard::new()).unwrap(),
                Player::Red => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 381, &BitBoard::new()).unwrap(),
                Player::Green => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 399, &BitBoard::new()).unwrap(),
            };

            (start, BitBoard::new())
        } else {
            let illegal = board.illegal(player, &self.placed);
            (board.corners(&illegal), illegal)
        };

        let mut moves = Vec::new(); 
        let bank = &self.banks[player as usize];
        for corner in corners.iter() {
            for (piece, taken) in bank.take_iter() {
                for orientation in piece.orientations.iter() {
                    for attachment in orientation.attachments.iter() {
                        if let Some(after) = board.place_shape(orientation, attachment, corner, &illegal) {
                            let mut copy = self.clone();
                            copy.placed[player as usize] = after;
                            copy.banks[player as usize] = taken.clone();
                            moves.push(copy);
                        }
                    }
                }
            }
        }
    
        moves
    }

    pub fn play_randomly<R: Rng>(&mut self, player: Player, rng: &mut R) -> bool {
        let moves = self.find_moves(player);
        if moves.is_empty() {
            false
        } else {
            *self = rng.choose(&moves).unwrap().clone();
            true
        }
    }

	pub fn find_winner(&self) -> Option<Player> {	
		let winner = if self.is_winner(Player::Blue) {
			Some(Player::Blue)
		} else if self.is_winner(Player::Yellow) {
			Some(Player::Yellow)
		} else if self.is_winner(Player::Red) {
			Some(Player::Red)
		} else if self.is_winner(Player::Yellow) {
			Some(Player::Yellow)
		} else {
			None
		};

		winner
	}

    pub fn display(&self) {
        for y in 0..20 {
            for x in 0..20 {
                let index = (19-y)*20 + x;
                if self.placed[0].is_occupied(index) {
                    print!("\x1b[106m");
                } else if self.placed[1].is_occupied(index) {
                    print!("\x1b[103m");
                } else if self.placed[2].is_occupied(index) {
                    print!("\x1b[101m");
				} else if self.placed[3].is_occupied(index) {
					print!("\x1b[102m");
                } else {
					print!("\x1b[100m");
				}

                print!(" ");
            }
            
            print!("\x1b[49m");
            println!();
        }
    }

    pub fn score(&self, player: Player) -> usize {
        self.placed[player as usize].count_tiles()
    }

    pub fn is_winner(&self, player: Player) -> bool {
		let mut turn = player;
		let score = self.score(player);
		for _ in 0..3 {
			turn = turn.next();
			if self.score(turn) > score {
				return false;
			}
		}

		true
    }
}

