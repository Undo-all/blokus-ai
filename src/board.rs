use bank::*;
use bitboard::*;
use rand::Rng;
use player::Player;
use pieces;

#[derive(Clone)]
pub struct Board {
    placed: [BitBoard; 2],
    banks: [Bank; 2],
}

impl Board {
    pub fn new() -> Self {
        Board {
            placed: [BitBoard::new(), BitBoard::new()],
            banks: [Bank::new(), Bank::new()],
        }
    }

    pub fn find_moves(&mut self, player: Player) -> Vec<Board> {
        let board = &self.placed[player as usize];
        
        let (corners, illegal) = if board.is_empty() {
            let start = match player {
                Player::Orange => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 130, &BitBoard::new()).unwrap(),
                Player::Purple => BitBoard::new().place_shape(&pieces::iter().nth(0).unwrap().orientations[0], &0, 65, &BitBoard::new()).unwrap(),
            };

            (start, BitBoard::new())
        } else {
            let illegal = board.illegal(&self.placed[player.opponent() as usize]);
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

    pub fn display(&self) {
        for y in 0..14 {
            for x in 0..14 {
                let index = (13-y)*14 + x;
                if self.placed[0].is_occupied(index) {
                    print!("\x1b[101m");
                } else if self.placed[1].is_occupied(index) {
                    print!("\x1b[106m");
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
        self.score(player) > self.score(player.opponent())
    }
}

