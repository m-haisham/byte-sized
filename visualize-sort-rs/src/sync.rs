use std::{
    sync::mpsc::{self, Receiver, SendError, TryRecvError},
    thread::{self, JoinHandle},
    time::Duration,
};

use notan::log::debug;
use rand::prelude::SliceRandom;

use crate::{
    algorithm::Algorithm,
    event::{Event, UpdateData},
    report::ReportVec,
};

pub struct SyncHandle {
    pub rx: Receiver<Event>,
    pub th: JoinHandle<Result<(), SendError<Event>>>,
}

pub struct SyncVec {
    name: String,

    vec: Vec<f32>,
    update: Option<UpdateData>,

    accesses: u32,
    writes: u32,

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

        let handle = Self::setup_thread(vec.clone(), algorithm);

        Self {
            name,
            vec,
            update: None,

            accesses: 0,
            writes: 0,

            handle: Some(handle),
        }
    }

    fn setup_thread<A>(vec: Vec<f32>, algorithm: A) -> SyncHandle
    where
        A: Algorithm + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        let th = thread::spawn(move || {
            let mut report = ReportVec::new(tx, vec);

            // FIXME: remove unwrap
            algorithm.sort(&mut report).unwrap();

            debug!("Sorting by {} completed.", algorithm.name());

            Ok(())
        });

        SyncHandle { rx, th }
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
}

impl SyncVec {
    pub fn try_apply(&mut self) {
        let handle = match &self.handle {
            Some(handle) => handle,
            None => return,
        };

        let event = match handle.rx.try_recv() {
            Ok(e) => e,
            Err(TryRecvError::Disconnected) => {
                // TODO: Handle disconnet.
                return;
            }
            Err(TryRecvError::Empty) => return,
        };

        let update = match event {
            Event::Update(data) => data,
            Event::Exit => {
                let handle = self.handle.take().unwrap();

                // FIXME: Handle thread join and output
                handle.th.join().expect("unable to join thread.").unwrap();
                return;
            }
        };

        match &update {
            UpdateData::Get { index: _ } => self.accesses += 1,
            UpdateData::Set { index, value } => {
                self.vec[*index] = *value;
                self.writes += 1;
            }
            UpdateData::Swap { index1, index2 } => {
                self.vec.swap(*index1, *index2);
                self.accesses += 2;
                self.writes += 2;
            }
        };

        self.update = Some(update);
    }
}

#[derive(Default)]
pub struct AccessLookup {
    pub accesses: Option<Vec<usize>>,
    pub writes: Option<Vec<usize>>,
}

impl SyncVec {
    pub fn lookup(&self) -> AccessLookup {
        let data = match self.update.as_ref() {
            Some(data) => data,
            None => return AccessLookup::default(),
        };

        match data {
            UpdateData::Get { index } => AccessLookup {
                accesses: Some(vec![*index]),
                writes: None,
            },
            UpdateData::Set { index, value: _ } => AccessLookup {
                accesses: None,
                writes: Some(vec![*index]),
            },
            UpdateData::Swap { index1, index2 } => AccessLookup {
                accesses: None,
                writes: Some(vec![*index1, *index2]),
            },
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
