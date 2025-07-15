pub mod record;

pub use record::{Event, Metadata, Record, Runtime, IOF};

use ptolemy::generated::observer;

pub struct RecordBatch {
    pub event: Vec<Event>,
    pub runtime: Vec<Runtime>,
    pub input: Vec<IOF>,
    pub output: Vec<IOF>,
    pub feedback: Vec<IOF>,
    pub metadata: Vec<Metadata>,
}

impl RecordBatch {
    pub fn new() -> Self {
        Self {
            event: Vec::new(),
            runtime: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            feedback: Vec::new(),
            metadata: Vec::new(),
        }
    }

    pub fn append_record(
        &mut self,
        record: observer::Record,
    ) -> Result<(), super::error::PtolemyError> {
        match Record::try_from(record)? {
            Record::Event(e) => self.event.push(e),
            Record::Runtime(r) => self.runtime.push(r),
            Record::Input(i) => self.input.push(i),
            Record::Output(o) => self.output.push(o),
            Record::Feedback(f) => self.feedback.push(f),
            Record::Metadata(m) => self.metadata.push(m),
        }

        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self.event.is_empty()
            & self.runtime.is_empty()
            & self.input.is_empty()
            & self.output.is_empty()
            & self.feedback.is_empty()
            & self.metadata.is_empty()
    }

    pub fn event(&mut self) -> Vec<Event> {
        self.event.drain(..).collect()
    }

    pub fn runtime(&mut self) -> Vec<Runtime> {
        self.runtime.drain(..).collect()
    }

    pub fn input(&mut self) -> Vec<IOF> {
        self.input.drain(..).collect()
    }

    pub fn output(&mut self) -> Vec<IOF> {
        self.output.drain(..).collect()
    }

    pub fn feedback(&mut self) -> Vec<IOF> {
        self.feedback.drain(..).collect()
    }

    pub fn metadata(&mut self) -> Vec<Metadata> {
        self.metadata.drain(..).collect()
    }

    pub fn flush_records(&mut self) -> Vec<Record> {
        let mut records = Vec::new();

        records.extend(self.event().into_iter().map(|i| Record::Event(i)));
        records.extend(self.runtime().into_iter().map(|i| Record::Runtime(i)));
        records.extend(self.input().into_iter().map(|i| Record::Input(i)));
        records.extend(self.output().into_iter().map(|i| Record::Output(i)));
        records.extend(self.feedback().into_iter().map(|i| Record::Feedback(i)));
        records.extend(self.metadata().into_iter().map(|i| Record::Metadata(i)));

        records
    }
}
