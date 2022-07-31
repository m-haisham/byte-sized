use std::sync::mpsc::{SendError, Sender};

use crate::event::{Event, UpdateData};

type ReportError = SendError<Event>;
type ReportResult<T> = Result<T, ReportError>;

pub trait ReportedIndex<T> {
    fn get(&self, index: usize) -> ReportResult<T>;
    fn set(&mut self, index: usize, value: T) -> ReportResult<()>;
    fn swap(&mut self, index1: usize, index2: usize) -> ReportResult<()>;
    fn exit(&self) -> ReportResult<()>;
}

pub struct ReportVec {
    sender: Sender<Event>,
    inner: Vec<f32>,
}

impl ReportVec {
    pub fn new(sender: Sender<Event>, inner: Vec<f32>) -> Self {
        Self { sender, inner }
    }
}

impl ReportedIndex<f32> for ReportVec {
    fn get(&self, index: usize) -> ReportResult<f32> {
        let temp = self.inner[index];
        self.sender.send(Event::Update(UpdateData::Get { index }))?;
        Ok(temp)
    }

    fn set(&mut self, index: usize, value: f32) -> ReportResult<()> {
        self.inner[index] = value;
        self.sender
            .send(Event::Update(UpdateData::Set { index, value }))?;
        Ok(())
    }

    fn swap(&mut self, index1: usize, index2: usize) -> ReportResult<()> {
        self.inner.swap(index1, index2);
        self.sender
            .send(Event::Update(UpdateData::Swap { index1, index2 }))?;
        Ok(())
    }

    fn exit(&self) -> ReportResult<()> {
        self.sender.send(Event::Exit)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{sync::mpsc, thread};

    use super::*;

    #[test]
    fn test_report_get() {
        let (tx, rx) = mpsc::channel();
        let thread_handle: thread::JoinHandle<Result<(), SendError<Event>>> =
            thread::spawn(move || {
                let mut report = ReportVec::new(tx, vec![1.0, 2.0, 3.0]);

                report.get(0)?;
                report.set(0, 1.0)?;
                report.swap(0, 1)?;
                report.exit()?;

                Ok(())
            });

        let events = rx.iter().collect::<Vec<Event>>();
        thread_handle
            .join()
            .expect("The thread panicked.")
            .expect("Failed to send event.");

        assert_eq!(
            events,
            vec![
                Event::Update(UpdateData::Get { index: 0 }),
                Event::Update(UpdateData::Set {
                    index: 0,
                    value: 1.0
                }),
                Event::Update(UpdateData::Swap {
                    index1: 0,
                    index2: 1,
                }),
                Event::Exit
            ]
        );
    }
}
