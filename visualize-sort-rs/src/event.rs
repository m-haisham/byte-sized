#[derive(Debug, PartialEq)]
pub struct Event {
    pub highlight: bool,
    pub kind: EventKind,
}

#[derive(Debug, PartialEq)]
pub enum EventKind {
    Start,
    Get { index: usize },
    Set { index: usize, value: f32 },
    Swap { a: usize, b: usize },
    Done,
}

impl Event {
    pub const START: Self = Event {
        highlight: false,
        kind: EventKind::Start,
    };

    pub const DONE: Self = Event {
        highlight: false,
        kind: EventKind::Done,
    };
}
