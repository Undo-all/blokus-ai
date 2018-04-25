use shape::Shape;

pub struct Piece {
    pub orientations: &'static [Shape],
}

pub type PieceId = u8;
