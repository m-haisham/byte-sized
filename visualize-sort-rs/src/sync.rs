use std::{
    sync::mpsc::{self, Receiver, SendError, TryRecvError},
    thread::{self, JoinHandle},
};

use notan::log::debug;
use rand::prelude::SliceRandom;

use crate::{
    algorithm::Algorithm,
    algorithms::algorithms,
    event::Event,
    report::{ReportVec, ReportedIndex},
};

type SyncHandle = JoinHandle<Result<(), SendError<Event>>>;

pub struct SyncVec {
    name: String,

    pub count: usize,
    pub index: usize,

    vec: Vec<f32>,
    event: Result<Event, String>,

    accesses: u32,
    writes: u32,

    rx: Receiver<Event>,
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
    pub fn new(count: usize, algorithm: usize) -> Self {
        let name = algorithms()[algorithm].name().clone();

        let mut vec = vec_uniform!(f32, count);
        vec.shuffle(&mut rand::thread_rng());

        let (rx, handle) = Self::setup_thread(vec.clone(), algorithm);

        Self {
            name,

            count,
            index: algorithm,

            vec,
            event: Ok(Event::Start),

            accesses: 0,
            writes: 0,

            rx,
            handle: Some(handle),
            done: false,
        }
    }

    fn setup_thread(vec: Vec<f32>, algorithm: usize) -> (Receiver<Event>, SyncHandle) {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut report = ReportVec::new(tx, vec);
            algorithms()[algorithm].sort(&mut report)?;

            report.send(Event::Done)?;
            debug!("Sorting by {} completed.", algorithms()[algorithm].name());

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
        self.vec.as_ref()
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
        let event = match self.rx.try_recv() {
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
                self.vec[*index] = *value;
                self.writes += 1;
            }
            Event::Swap { index1, index2 } => {
                self.vec.swap(*index1, *index2);
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
            Event::Swap { index1, index2 } => AccessLookup {
                accesses: None,
                writes: Some(vec![*index1, *index2]),
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
