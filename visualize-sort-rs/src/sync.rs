use std::{
    sync::mpsc::{self, Receiver, SendError, TryRecvError},
    thread::{self, JoinHandle},
};

use notan::log::debug;
use rand::prelude::SliceRandom;

use crate::{algorithms, emit::EmitVec, event::Event};

type SyncHandle = JoinHandle<Result<(), SendError<Event>>>;

#[derive(Debug)]
pub struct SyncVec {
    name: String,

    pub count: usize,
    pub index: usize,

    values: Vec<f32>,
    event: Result<Event, String>,

    accesses: u32,
    writes: u32,

    rx: Option<Receiver<Event>>,
    handle: Option<SyncHandle>,
    done: bool,
}

macro_rules! vec_uniform {
    ($t:ty, $c:expr) => {{
        let mut output = Vec::<$t>::new();
        let segment_value = 1.0 / $c as $t;
        for offset in 0..$c {
            output.push(segment_value * offset as $t);
        }
        output
    }};
}

impl SyncVec {
    pub fn new(count: usize, index: usize) -> Self {
        let name = algorithms::ALGORITHMS[index].name().clone();

        let mut values = vec_uniform!(f32, count);
        values.shuffle(&mut rand::thread_rng());

        Self {
            name,

            count,
            index,

            values,
            event: Ok(Event::Start),

            accesses: 0,
            writes: 0,

            rx: None,
            handle: None,
            done: false,
        }
    }

    fn setup_thread(&self) -> (Receiver<Event>, SyncHandle) {
        let (tx, rx) = mpsc::channel();

        let mut values = self.values.clone();
        let index = self.index;

        let handle = thread::spawn(move || {
            let algorithm = &algorithms::ALGORITHMS[index];

            debug!("Sorting by {}", algorithm.name());

            let mut vec = EmitVec::borrow(&tx, &mut values, 0);
            algorithm.sort(&mut vec)?;
            tx.send(Event::Done)?;

            debug!("Sorting by {} completed.", algorithm.name());

            Ok(())
        });

        (rx, handle)
    }
}

impl SyncVec {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn values(&self) -> &[f32] {
        self.values.as_ref()
    }

    pub fn accesses(&self) -> u32 {
        self.accesses
    }

    pub fn writes(&self) -> u32 {
        self.writes
    }

    pub fn done(&self) -> bool {
        self.done
    }
}

impl SyncVec {
    pub fn next(&mut self) {
        let rx = match self.rx.as_ref() {
            Some(rx) => rx,
            None => {
                let (rx, handle) = self.setup_thread();
                self.rx = Some(rx);
                self.handle = Some(handle);
                self.rx.as_ref().unwrap()
            }
        };

        let event = match rx.try_recv() {
            Ok(e) => e,
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                self.disconnected();
                return;
            }
        };

        match &event {
            Event::Get { index: _ } => self.accesses += 1,
            Event::Set { index, value } => {
                self.values[*index] = *value;
                self.writes += 1;
            }
            Event::Swap { a, b } => {
                self.values.swap(*a, *b);
                self.accesses += 2;
                self.writes += 2;
            }
            Event::Start | Event::Done => (),
        };

        self.event = Ok(event);
    }

    fn disconnected(&mut self) {
        let handle = match self.handle.take() {
            Some(handle) => handle,
            None => return,
        };

        let result = match handle.join() {
            Ok(r) => r,
            Err(_) => {
                self.event = Err(String::from("Error closing disconnected thread."));
                return;
            }
        };

        if let Err(_) = result {
            self.event = Err(String::from("Error receiving data"));
        }

        self.done = true;
        debug!("{} thread closed.", self.name());
    }
}

#[derive(Default)]
pub struct AccessLookup {
    pub accesses: Option<Vec<usize>>,
    pub writes: Option<Vec<usize>>,
}

impl SyncVec {
    pub fn lookup(&self) -> AccessLookup {
        let data = match self.event.as_ref() {
            Ok(data) => data,
            Err(_) => return AccessLookup::default(),
        };

        match data {
            Event::Get { index } => AccessLookup {
                accesses: Some(vec![*index]),
                writes: None,
            },
            Event::Set { index, value: _ } => AccessLookup {
                accesses: None,
                writes: Some(vec![*index]),
            },
            Event::Swap { a, b } => AccessLookup {
                accesses: None,
                writes: Some(vec![*a, *b]),
            },
            Event::Start | Event::Done => AccessLookup::default(),
        }
    }
}

impl AccessLookup {
    pub fn accesses_contains(&self, index: &usize) -> bool {
        match &self.accesses {
            Some(value) if value.contains(index) => true,
            Some(_) => false,
            None => false,
        }
    }

    pub fn writes_contains(&self, index: &usize) -> bool {
        match &self.writes {
            Some(value) if value.contains(index) => true,
            Some(_) => false,
            None => false,
        }
    }
}
