use crate::event::Event;
use std::sync::mpsc::{SendError, Sender};

type ReportError = SendError<Event>;
type ReportResult<T> = Result<T, ReportError>;

pub trait ReportedIndex<T> {
    fn send(&self, event: Event) -> ReportResult<()>;
    fn get(&self, index: usize) -> ReportResult<T>;
    fn set(&mut self, index: usize, value: T) -> ReportResult<()>;
    fn swap(&mut self, index1: usize, index2: usize) -> ReportResult<()>;
    fn len(&self) -> usize;
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
    fn send(&self, event: Event) -> ReportResult<()> {
        self.sender.send(event)
    }

    fn get(&self, index: usize) -> ReportResult<f32> {
        let temp = self.inner[index];
        self.sender.send(Event::Get { index })?;
        Ok(temp)
    }

    fn set(&mut self, index: usize, value: f32) -> ReportResult<()> {
        self.inner[index] = value;
        self.sender.send(Event::Set { index, value })?;
        Ok(())
    }

    fn swap(&mut self, index1: usize, index2: usize) -> ReportResult<()> {
        self.inner.swap(index1, index2);
        self.sender.send(Event::Swap { index1, index2 })?;
        Ok(())
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
pub struct TestVec<T>(pub Vec<T>);

#[cfg(test)]
impl<T: Copy> ReportedIndex<T> for TestVec<T> {
    fn send(&self, _: Event) -> ReportResult<()> {
        Ok(())
    }

    fn get(&self, index: usize) -> ReportResult<T> {
        Ok(self.0[index])
    }

    fn set(&mut self, index: usize, value: T) -> ReportResult<()> {
        self.0[index] = value;
        Ok(())
    }

    fn swap(&mut self, index1: usize, index2: usize) -> ReportResult<()> {
        self.0.swap(index1, index2);
        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
pub mod tests {
    use std::{sync::mpsc, thread};

    use super::*;

    #[test]
    fn test_report() {
        let (tx, rx) = mpsc::channel();
        let thread_handle: thread::JoinHandle<Result<(), SendError<Event>>> =
            thread::spawn(move || {
                let mut report = ReportVec::new(tx, vec![1.0, 2.0, 3.0]);

                report.get(0)?;
                report.set(0, 1.0)?;
                report.swap(0, 1)?;

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
                Event::Get { index: 0 },
                Event::Set {
                    index: 0,
                    value: 1.0
                },
                Event::Swap {
                    index1: 0,
                    index2: 1,
                },
            ]
        );
    }
}
