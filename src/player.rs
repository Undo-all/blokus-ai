#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Orange = 0,
    Purple = 1,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::Orange => Player::Purple,
            Player::Purple => Player::Orange,
        }
    }
}
