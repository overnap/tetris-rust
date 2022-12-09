#[derive(Clone, Copy, PartialEq)]
pub enum InputType {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCW,
    RotateCCW,
    Flip,
    Hold,
    SIZE
}