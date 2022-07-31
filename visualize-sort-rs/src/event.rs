#[derive(Debug, PartialEq)]
pub enum Event {
    Update(UpdateData),
    Exit,
}

#[derive(Debug, PartialEq)]
pub enum UpdateData {
    Get { index: usize },
    Set { index: usize, value: f32 },
    Swap { index1: usize, index2: usize },
}
