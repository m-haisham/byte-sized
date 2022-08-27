use crate::event::{Event, EventKind};
use std::{
    ops::{Deref, DerefMut, Range},
    sync::mpsc::{SendError, Sender},
};

pub type EmitResult<T> = Result<T, SendError<Event>>;

pub struct EmitVec<'a> {
    pub tx: EmitSender<'a>,
    pub array: EmitArray<'a>,
    pub offset: usize,
    pub highlight: bool,
}

impl<'a> EmitVec<'a> {
    #[inline]
    pub fn borrow(tx: &'a Sender<Event>, array: &'a mut [f32], offset: usize) -> Self {
        Self {
            tx: EmitSender::Borrowed(tx),
            array: EmitArray::Borrowed(array),
            offset,
            highlight: true,
        }
    }

    #[inline]
    pub fn new(tx: Sender<Event>, array: Vec<f32>, offset: usize) -> Self {
        Self {
            tx: EmitSender::Owned(tx),
            array: EmitArray::Owned(array),
            offset,
            highlight: true,
        }
    }

    #[inline]
    pub fn set_highlight(&mut self, value: bool) {
        self.highlight = value;
    }

    pub fn send(&self, kind: EventKind) -> EmitResult<()> {
        self.tx.send(Event {
            highlight: self.highlight,
            kind,
        })
    }

    pub fn get(&self, index: usize) -> EmitResult<f32> {
        let value = self.array[index];
        self.send(EventKind::Get {
            index: index + self.offset,
        })?;
        Ok(value)
    }

    pub fn set(&mut self, index: usize, value: f32) -> EmitResult<()> {
        self.array[index] = value;
        self.send(EventKind::Set {
            index: index + self.offset,
            value,
        })?;
        Ok(())
    }

    pub fn swap(&mut self, a: usize, b: usize) -> EmitResult<()> {
        self.array.swap(a, b);
        self.send(EventKind::Swap {
            a: a + self.offset,
            b: b + self.offset,
        })?;
        Ok(())
    }

    pub fn slice(&mut self, range: Range<usize>) -> EmitResult<EmitVec> {
        let offset = self.offset + range.start;
        let vec = EmitVec::borrow(&self.tx, &mut self.array[range], offset);
        Ok(vec)
    }

    pub fn clone_slice<'b>(&self, range: Range<usize>) -> EmitResult<EmitVec<'b>> {
        let offset = self.offset + range.start;

        let mut array = Vec::new();
        array.extend_from_slice(&self.array[range]);

        let vec = EmitVec::new(self.tx.clone(), array, offset);
        Ok(vec)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.array.len()
    }
}

pub enum EmitSender<'a> {
    Borrowed(&'a Sender<Event>),
    Owned(Sender<Event>),
}

impl Deref for EmitSender<'_> {
    type Target = Sender<Event>;

    fn deref(&self) -> &Self::Target {
        match self {
            EmitSender::Borrowed(s) => s,
            EmitSender::Owned(s) => s,
        }
    }
}

pub enum EmitArray<'a> {
    Borrowed(&'a mut [f32]),
    Owned(Vec<f32>),
}

impl Deref for EmitArray<'_> {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        match self {
            EmitArray::Borrowed(a) => a,
            EmitArray::Owned(a) => a,
        }
    }
}

impl DerefMut for EmitArray<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            EmitArray::Borrowed(a) => a,
            EmitArray::Owned(a) => a,
        }
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
                let mut vec = EmitVec::borrow(&tx, &mut array, 0);

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

        macro_rules! event {
            ($kind:expr) => {
                Event {
                    highlight: true,
                    kind: $kind,
                }
            };
        }

        assert_eq!(
            events,
            vec![
                event!(EventKind::Get { index: 0 }),
                event!(EventKind::Set {
                    index: 0,
                    value: 1.0
                }),
                event!(EventKind::Swap { a: 0, b: 1 }),
            ]
        );
    }
}
