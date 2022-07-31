#[derive(Debug)]
pub enum Event {
    Update(UpdateData),
    Exit,
}

#[derive(Debug)]
pub enum UpdateData {
    Get { index: u32 },
    Set { index: u32, value: f32 },
    Swap { index1: u32, index2: u32 },
}
