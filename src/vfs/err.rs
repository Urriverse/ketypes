#[repr(usize)]
#[derive(Debug)]
pub enum KeFsError {
    Unknown         = 0,
    NotAFile        = 1,
    OutOfBounds     = 2,
    NoEntry         = 3,
    NotADirectory   = 4,
    Found           = 5,
    AlreadyExists   = 6,
    InvalidPath     = 7,
    NotMounted      = 8,
    NotEmpty        = 9,
}
