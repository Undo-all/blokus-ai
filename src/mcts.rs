use board::*;
use player::*;
use player_set::*;

use rand::Rng;

const EXPLOITATION: f32 = 1.5;
const EPSILON: f32 = 0.0001;

#[derive(Clone)]
pub struct Node {
    pub board: Board,
    turn: Player,
    visits: u32,
    wins: [u32; 4],
    terminal: bool,
	out: PlayerSet,
    children: Vec<Node>,
}

impl Node {
    pub fn new(board: Board, turn: Player, out: PlayerSet) -> Self {
        Node {
            board: board,
            turn: turn,
            visits: 0,
            wins: [0, 0, 0, 0],
            terminal: false,
			out: out,
			//out: PlayerSet::new(),
            children: Vec::new(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn expand(&mut self) {
        if self.terminal {
            return;
        }

        let moves = self.board.find_moves(self.turn);

        if moves.is_empty() {
			let mut out = self.out.add(self.turn);

			let mut turn = self.turn;
			for _ in 0..3 {
				turn = self.turn.next();
				
				if out.contains(turn) {
					continue;
				}

				let moves = self.board.find_moves(turn);
				if moves.is_empty() {
					out = out.add(turn);
				} else {
					self.out = out.clone();
					for board in moves {
						self.children.push(Node::new(board, turn, out.clone()));
					}

					return;
				}
			}
			
			self.out = out;
			self.terminal = true;
        } else {
            let turn = self.turn.next();

            for board in moves {
                self.children.push(Node::new(board, turn, self.out.clone()));
            }
        }
    }
    
    fn select(&mut self) -> &mut Self {
        let root_visits = self.visits as f32;
		
        let utc = |child: &Node| {
			let wins = child.wins[self.turn as usize] as f32;

            let visits = (child.visits as f32) + EPSILON;
            let mean = wins / visits;
            mean + EXPLOITATION * ((root_visits + 1.0).ln() / visits).sqrt()
        };

		let index = {
			let (i, n) = self.children.iter().enumerate().max_by(|(i, a), (j, b)| utc(a).partial_cmp(&utc(b)).unwrap()).unwrap();
			i
		};
			
		&mut self.children[index]
        //self.children.iter_mut().max_by(|a, b| utc(a.wins[self.turn as usize] as f32, a.visits as f32).partial_cmp(&utc(b.wins[self.turn as usize] as f32, b.visits as f32)).unwrap()).unwrap()
    }
    
    fn playout<R: Rng>(&mut self, rng: &mut R) -> Player {
        let mut turn = self.turn;
        let mut board = self.board.clone();
		let mut out = self.out.clone();

		while !out.is_full() {
			turn = turn.next();
			if out.contains(turn) {
				continue;
			}

			if !board.play_randomly(turn, rng) {
				out = out.add(turn);
			}
		}

		let winner = match self.board.find_winner() {
			Some(player) => player,
			None => rng.gen(),
		}; 

		self.visits += 1;
		self.wins[winner as usize] += 1;

        winner
    }

    pub fn step<R: Rng>(&mut self, rng: &mut R) -> Player {
        if self.is_leaf() {
            self.expand();
        }

		/*
        if self.terminal {
            return if self.board.is_winner(self.turn) {
                true
            } else if self.board.is_winner(self.turn.opponent()) {
                false
            } else {
                rng.gen()
            };
        }
		*/

		if self.terminal {
			return if let Some(winner) = self.board.find_winner() {
				winner
			} else {
				rng.gen()
			};
		}
        
        let child = self.select();
        let winner = if child.visits == 0 {
            child.playout(rng)
        } else {
            child.step(rng)
        };

		self.visits += 1;
		self.wins[winner as usize] += 1;
        
        return winner;
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal
    }

    pub fn display(&self) {
        self.board.display();
    }

    pub fn best_child(&self) -> &Self {
        self.children.iter().max_by_key(|n| n.visits).unwrap()
    }
}

