use std::sync::mpsc::SendError;

use crate::{event::Event, report::ReportedIndex};

pub trait Algorithm {
    fn name(&self) -> String;
    fn sort(&self, source: &mut impl ReportedIndex<f32>) -> Result<(), SendError<Event>>;
}
