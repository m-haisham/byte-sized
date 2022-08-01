#[derive(Debug, PartialEq)]
pub enum Event {
    Start,
    Get { index: usize },
    Set { index: usize, value: f32 },
    Swap { index1: usize, index2: usize },
    Done,
}
