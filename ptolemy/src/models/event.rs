use crate::models::json_serializable::{JsonSerializable, Parameters};
use crate::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, MetadataRecord, OutputRecord,
    Record, RuntimeRecord, Tier,
};
use crate::models::id::Id;

pub trait Proto {
    fn proto(&self) -> RecordData;
}

#[derive(Clone, Debug)]
pub struct ProtoEvent {
    pub name: String,
    pub parameters: Option<Parameters>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

impl ProtoEvent {
    pub fn new(
        name: String,
        parameters: Option<Parameters>,
        version: Option<String>,
        environment: Option<String>,
    ) -> Self {
        Self {
            name,
            parameters,
            version,
            environment,
        }
    }
}

impl Proto for ProtoEvent {
    fn proto(&self) -> RecordData {
        let name = self.name.clone();
        let parameters = match &self.parameters {
            Some(p) => Some(p.clone().into()),
            None => None,
        };

        let version = self.version.clone();
        let environment = self.environment.clone();

        RecordData::Event(EventRecord {
            name,
            parameters,
            version,
            environment,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoRuntime {
    pub start_time: f32,
    pub end_time: f32,
    pub error_type: Option<String>,
    pub error_content: Option<String>,
}

impl ProtoRuntime {
    pub fn new(
        start_time: f32,
        end_time: f32,
        error_type: Option<String>,
        error_content: Option<String>,
    ) -> Self {
        Self {
            start_time,
            end_time,
            error_type,
            error_content,
        }
    }
}

impl Proto for ProtoRuntime {
    fn proto(&self) -> RecordData {
        RecordData::Runtime(RuntimeRecord {
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type.clone(),
            error_content: self.error_content.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoInput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoInput {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoInput {
    fn proto(&self) -> RecordData {
        RecordData::Input(InputRecord {
            field_name: self.field_name.clone(),
            field_value: Some(self.field_value.clone().into()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoOutput {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoOutput {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoOutput {
    fn proto(&self) -> RecordData {
        RecordData::Output(OutputRecord {
            field_name: self.field_name.clone(),
            field_value: Some(self.field_value.clone().into()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoFeedback {
    pub field_name: String,
    pub field_value: JsonSerializable,
}

impl ProtoFeedback {
    pub fn new(field_name: String, field_value: JsonSerializable) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoFeedback {
    fn proto(&self) -> RecordData {
        RecordData::Feedback(FeedbackRecord {
            field_name: self.field_name.clone(),
            field_value: Some(self.field_value.clone().into()),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoMetadata {
    pub field_name: String,
    pub field_value: String,
}

impl ProtoMetadata {
    pub fn new(field_name: String, field_value: String) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl Proto for ProtoMetadata {
    fn proto(&self) -> RecordData {
        RecordData::Metadata(MetadataRecord {
            field_name: self.field_name.clone(),
            field_value: self.field_value.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ProtoRecord<T: Proto> {
    pub tier: Tier,
    pub parent_id: Id,
    pub id: Id,

    pub record_data: T,
}

impl<T: Proto> ProtoRecord<T> {
    pub fn new(tier: Tier, parent_id: Id, id: Id, record_data: T) -> Self {
        Self {
            tier,
            parent_id,
            id,
            record_data,
        }
    }

    pub fn proto(&self) -> Record {
        Record {
            tier: self.tier.into(),
            parent_id: self.parent_id.to_string(),
            id: self.id.to_string(),
            record_data: Some(self.record_data.proto()),
        }
    }
}
