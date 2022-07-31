use std::sync::mpsc::SendError;

use crate::{report::ReportedIndex, event::Event};

pub trait Algorithm {
    fn name() -> String;
    fn sort(source: &mut impl ReportedIndex<f32>) -> Result<(), SendError<Event>>;
}
