use crate::event::{
    ProtoEvent,
    ProtoFeedback,
    ProtoInput,
    ProtoMetadata,
    ProtoOutput,
    ProtoRecord,
    ProtoRuntime,
};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PtolemyClientState {
    pub event: Option<ProtoRecord<ProtoEvent>>,
    pub runtime: Option<ProtoRecord<ProtoRuntime>>,
    pub input: Option<Vec<ProtoRecord<ProtoInput>>>,
    pub output: Option<Vec<ProtoRecord<ProtoOutput>>>,
    pub feedback: Option<Vec<ProtoRecord<ProtoFeedback>>>,
    pub metadata: Option<Vec<ProtoRecord<ProtoMetadata>>>,
    pub start_time: Option<f32>,
    pub end_time: Option<f32>,
}

impl PtolemyClientState {
    pub fn new() -> Self {
        Self {
            event: None,
            runtime: None,
            input: None,
            output: None,
            feedback: None,
            metadata: None,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        match self.start_time.is_none() {
            true => {
                // set start time to current time in f32
                self.start_time = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f32
                        / 1000.0,
                );
            }
            false => {
                panic!("Start time already set!");
            }
        }
    }

    pub fn end(&mut self) {
        match self.end_time.is_none() {
            true => {
                // set end time to current time in f32
                self.end_time = Some(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f32
                        / 1000.0,
                );
            }
            false => {
                panic!("End time already set!");
            }
        }
    }

    pub fn set_event(&mut self, event: ProtoRecord<ProtoEvent>) {
        self.event = Some(event);
    }

    pub fn set_runtime(&mut self, runtime: ProtoRecord<ProtoRuntime>) {
        self.runtime = Some(runtime);
    }

    pub fn set_input(&mut self, input: Vec<ProtoRecord<ProtoInput>>) {
        self.input = Some(input);
    }

    pub fn set_output(&mut self, output: Vec<ProtoRecord<ProtoOutput>>) {
        self.output = Some(output);
    }

    pub fn set_feedback(&mut self, feedback: Vec<ProtoRecord<ProtoFeedback>>) {
        self.feedback = Some(feedback);
    }

    pub fn set_metadata(&mut self, metadata: Vec<ProtoRecord<ProtoMetadata>>) {
        self.metadata = Some(metadata);
    }

    pub fn event_id(&self) -> PyResult<Uuid> {
        match &self.event {
            Some(event) => Ok(event.id),
            None => Err(PyValueError::new_err("No event set!")),
        }
    }
}
