use rand::{distributions::*, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Blue = 0,
    Yellow = 1,
    Red = 2,
    Green = 3,
}

impl Player {
    pub fn next(self) -> Self {
        match self {
            Player::Blue => Player::Yellow,
            Player::Yellow => Player::Red,
            Player::Red => Player::Green,
            Player::Green => Player::Blue,
        }
    }
}

pub struct PlayerIterator {
    turn: Player,
    done: bool,
}

impl PlayerIterator {
    fn new() -> Self {
        PlayerIterator {
            turn: Player::Blue,
            done: false,
        }
    }
}

impl Iterator for PlayerIterator {
    type Item = Player;

    fn next(&mut self) -> Option<Player> {
        if self.done {
            return None;
        }

        let turn = self.turn;
        self.turn = self.turn.next();
        if self.turn == Player::Blue {
            self.done = true;
        }

        Some(turn)
    }
}

pub fn iter() -> PlayerIterator {
    PlayerIterator::new()
}

impl Distribution<Player> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
        *rng.choose(&[Player::Blue, Player::Yellow, Player::Red, Player::Green])
            .unwrap()
    }
}
