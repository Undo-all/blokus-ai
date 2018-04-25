#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    Orange = 0,
    Purple = 1,
}

impl Player {
    fn opponent(self) -> Self {
        match self {
            Player::Orange => Player::Purple,
            Player::Purple => Player::Orange,
        }
    }
}
