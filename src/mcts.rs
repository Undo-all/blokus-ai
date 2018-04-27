use board::*;
use player::*;

use rand::Rng;

const EXPLOITATION: f32 = 1.5;
const EPSILON: f32 = 0.0001;

#[derive(Clone)]
pub struct Node {
    pub board: Board,
    turn: Player,
    visits: u32,
    wins: u32,
    packing: bool,
    terminal: bool,
    children: Vec<Node>,
}

impl Node {
    pub fn new(board: Board, turn: Player, packing: bool) -> Self {
        Node {
            board: board,
            turn: turn,
            visits: 0,
            wins: 0,
            packing: packing,
            terminal: false,
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
            let opponent = self.turn.opponent();
            let moves = self.board.find_moves(opponent);
            if moves.is_empty() {
                self.terminal = true;
            } else {
                for board in moves {
                    self.children.push(Node::new(board, opponent, true));
                }
            }
        } else {
            let turn = if self.packing {
                self.turn
            } else {
                self.turn.opponent()
            };

            for board in moves {
                self.children.push(Node::new(board, turn, self.packing));
            }
        }
    }

    fn select(&mut self) -> &mut Self {
        let root_visits = self.visits as f32;
        let packing = self.packing;

        let utc = |child: &Node| {
            let wins = if packing {
                child.wins
            } else {
                (child.visits - child.wins) 
            } as f32;

            let visits = (child.visits as f32) + EPSILON;
            let mean = wins / visits;
            mean + EXPLOITATION * ((root_visits + 1.0).ln() / visits).sqrt()
        };

        self.children.iter_mut().max_by(|a, b| utc(a).partial_cmp(&utc(b)).unwrap()).unwrap()
    }

    fn playout<R: Rng>(&mut self, rng: &mut R) -> bool {
        let mut turn = self.turn;
        let mut board = self.board.clone();

        while board.play_randomly(turn, rng) {
            turn = turn.opponent();
        }
    
        turn = turn.opponent();
        while board.play_randomly(turn, rng) {}

        let win = if self.board.is_winner(self.turn) {
            true
        } else if self.board.is_winner(self.turn.opponent()) {
            false
        } else {
            rng.gen()
        };

        self.wins += win as u32;
        self.visits += 1;
        win
    }

    pub fn step<R: Rng>(&mut self, rng: &mut R) -> bool {
        if self.is_leaf() {
            self.expand();
        }

        if self.terminal {
            return if self.board.is_winner(self.turn) {
                true
            } else if self.board.is_winner(self.turn.opponent()) {
                false
            } else {
                rng.gen()
            };
        }
        
        let child = self.select();
        let win = if child.visits == 0 {
            child.playout(rng)
        } else {
            child.step(rng)
        };
        
        self.wins += !win as u32;
        self.visits += 1;
        return !win;
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

