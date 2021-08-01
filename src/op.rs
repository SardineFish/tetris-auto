#[derive(Clone, Copy)]
pub enum GameOP {
    New,
    Left(i8),
    Right(i8),
    Down(i8),
    Rotate(i8),
}