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

impl Distribution<Player> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Player {
		*rng.choose(&[Player::Blue, Player::Yellow, Player::Red, Player::Green]).unwrap()
	}
}
