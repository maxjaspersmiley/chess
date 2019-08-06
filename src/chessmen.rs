#[derive(Debug, Copy, Clone)]
pub trait Chessman: Sized, Copy {
    fn move_to(&self, new_square: (u8, u8)) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct Pawn {
    square: (u8, u8),
}

#[derive(Debug, Copy, Clone)]
impl Chessman for Pawn {
    fn move_to(&self, new_square: (u8, u8)) -> bool {
        true
    }
}

impl Sized for Chessman {}
