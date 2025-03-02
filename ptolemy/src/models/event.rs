use chrono::{DateTime, NaiveDateTime};

use crate::error::ParseError;
use crate::generated::observer::{
    record::RecordData, EventRecord, FeedbackRecord, InputRecord, MetadataRecord, OutputRecord,
    Record, RuntimeRecord, Tier,
};
use crate::models::json::JSON;
use crate::models::Id;

pub trait Proto: TryFrom<RecordData, Error = ParseError> {
    fn proto(&self) -> RecordData;
}

#[derive(Clone, Debug)]
pub struct ProtoEvent {
    pub name: String,
    pub parameters: Option<JSON>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

impl ProtoEvent {
    pub fn new(
        name: String,
        parameters: Option<JSON>,
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

impl TryFrom<RecordData> for ProtoEvent {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Event(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoEvent {
            name: val.name,
            parameters: match val.parameters {
                Some(p) => Some(p.try_into()?),
                None => None,
            },
            version: val.version,
            environment: val.environment,
        })
    }
}

impl Proto for ProtoEvent {
    fn proto(&self) -> RecordData {
        let name = self.name.clone();
        let parameters = self.parameters.as_ref().map(|p| p.clone().into());

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

    pub fn start_time(&self) -> NaiveDateTime {
        DateTime::from_timestamp(
            self.start_time.trunc() as i64,
            (self.start_time.fract() * 1e9) as u32,
        )
        .unwrap()
        .naive_utc()
    }

    pub fn end_time(&self) -> NaiveDateTime {
        DateTime::from_timestamp(
            self.end_time.trunc() as i64,
            (self.end_time.fract() * 1e9) as u32,
        )
        .unwrap()
        .naive_utc()
    }
}

impl TryFrom<RecordData> for ProtoRuntime {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Runtime(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoRuntime {
            start_time: val.start_time,
            end_time: val.end_time,
            error_type: val.error_type,
            error_content: val.error_content,
        })
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
    pub field_value: JSON,
}

impl ProtoInput {
    pub fn new(field_name: String, field_value: JSON) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl TryFrom<RecordData> for ProtoInput {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Input(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoInput {
            field_name: val.field_name,
            field_value: val.field_value.unwrap().try_into()?,
        })
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
    pub field_value: JSON,
}

impl ProtoOutput {
    pub fn new(field_name: String, field_value: JSON) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl TryFrom<RecordData> for ProtoOutput {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Output(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoOutput {
            field_name: val.field_name,
            field_value: val.field_value.unwrap().try_into()?,
        })
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
    pub field_value: JSON,
}

impl ProtoFeedback {
    pub fn new(field_name: String, field_value: JSON) -> Self {
        Self {
            field_name,
            field_value,
        }
    }
}

impl TryFrom<RecordData> for ProtoFeedback {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Feedback(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoFeedback {
            field_name: val.field_name,
            field_value: val.field_value.unwrap().try_into()?,
        })
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

impl TryFrom<RecordData> for ProtoMetadata {
    type Error = crate::error::ParseError;

    fn try_from(value: RecordData) -> Result<Self, Self::Error> {
        let val = match value {
            RecordData::Metadata(e) => e,
            _ => return Err(crate::error::ParseError::InvalidType),
        };

        Ok(ProtoMetadata {
            field_name: val.field_name,
            field_value: val.field_value,
        })
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

impl<T: Proto> TryFrom<Record> for ProtoRecord<T> {
    type Error = crate::error::ParseError;

    fn try_from(value: Record) -> Result<Self, Self::Error> {
        let tier = value.tier();
        let parent_id: Id = value.parent_id.try_into()?;
        let id: Id = value.id.try_into()?;
        let record_data: T = TryInto::<T>::try_into(
            value
                .record_data
                .ok_or(crate::error::ParseError::InvalidType)?,
        )?;

        Ok(ProtoRecord::<T> {
            tier,
            parent_id,
            id,
            record_data,
        })
    }
}
