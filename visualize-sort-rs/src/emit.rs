use crate::event::Event;
use std::sync::mpsc::{SendError, Sender};

pub type EmitResult<T> = Result<T, SendError<Event>>;

pub struct EmitVec<'a> {
    tx: &'a Sender<Event>,
    array: &'a mut [f32],
    offset: usize,
}

impl<'a> EmitVec<'a> {
    #[inline(always)]
    pub fn new(tx: &'a Sender<Event>, array: &'a mut [f32], offset: usize) -> Self {
        Self { tx, array, offset }
    }

    pub fn get(&self, index: usize) -> EmitResult<f32> {
        let value = self.array[index];
        self.tx.send(Event::Get { index })?;
        Ok(value)
    }

    pub fn set(&mut self, index: usize, value: f32) -> EmitResult<()> {
        self.array[index] = value;
        self.tx.send(Event::Set { index, value })?;
        Ok(())
    }

    pub fn swap(&mut self, a: usize, b: usize) -> EmitResult<()> {
        self.array.swap(a, b);
        self.tx.send(Event::Swap { a, b })?;
        Ok(())
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.array.len()
    }
}

#[cfg(test)]
pub mod tests {
    use std::{sync::mpsc, thread};

    use super::*;

    #[test]
    fn test_emit() {
        let (tx, rx) = mpsc::channel();
        let thread_handle: thread::JoinHandle<Result<(), SendError<Event>>> =
            thread::spawn(move || {
                let mut array = [1.0, 2.0, 3.0];
                let mut vec = EmitVec::new(&tx, &mut array, 0);

                vec.get(0)?;
                vec.set(0, 1.0)?;
                vec.swap(0, 1)?;

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
                Event::Swap { a: 0, b: 1 },
            ]
        );
    }
}
