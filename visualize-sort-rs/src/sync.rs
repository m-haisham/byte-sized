use std::{
    sync::mpsc::{self, Receiver, SendError, TryRecvError},
    thread::{self, JoinHandle},
};

use notan::log::debug;
use rand::prelude::SliceRandom;

use crate::{
    algorithm::Algorithm,
    event::Event,
    report::{ReportVec, ReportedIndex},
};

type SyncHandle = JoinHandle<Result<(), SendError<Event>>>;

pub struct SyncVec {
    name: String,

    vec: Vec<f32>,
    event: Result<Event, String>,

    accesses: u32,
    writes: u32,

    rx: Receiver<Event>,
    handle: Option<SyncHandle>,
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
    pub fn new<A>(count: usize, algorithm: A) -> Self
    where
        A: Algorithm + Send + 'static,
    {
        let name = algorithm.name().clone();

        let mut vec = vec_uniform!(f32, count);
        vec.shuffle(&mut rand::thread_rng());

        let (rx, handle) = Self::setup_thread(vec.clone(), algorithm);

        Self {
            name,
            vec,
            event: Ok(Event::Start),

            accesses: 0,
            writes: 0,

            rx,
            handle: Some(handle),
        }
    }

    fn setup_thread<A>(vec: Vec<f32>, algorithm: A) -> (Receiver<Event>, SyncHandle)
    where
        A: Algorithm + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            let mut report = ReportVec::new(tx, vec);
            algorithm.sort(&mut report)?;

            report.send(Event::Done)?;
            debug!("Sorting by {} completed.", algorithm.name());

            Ok(())
        });

        (rx, handle)
    }
}

impl SyncVec {
    pub fn name(&self) -> &str {
        self.name.as_ref()
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
        match self.event.as_ref() {
            Ok(event) => matches!(event, Event::Done),
            Err(_) => true,
        }
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
