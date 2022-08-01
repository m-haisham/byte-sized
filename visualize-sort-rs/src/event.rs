#[derive(Debug, PartialEq)]
pub enum Event {
    Start,
    Get { index: usize },
    Set { index: usize, value: f32 },
    Swap { a: usize, b: usize },
    Done,
}
