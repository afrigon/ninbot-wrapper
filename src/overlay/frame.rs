#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    Empty,
    Image(Vec<u8>)
}
