use std::collections::BTreeMap;
use prost_types::value::Kind;
use prost_types::{ListValue, Struct, Value};
use crate::generated::observer::{PublishRequest, PublishResponse, Record, Tier, LogType, observer_client::ObserverClient};
use tonic::transport::Channel;
use pyo3::prelude::*;

pub mod client_config {
    pub struct ObserverConfig {
        pub host: String,
        pub port: String
    }

    impl ObserverConfig {
        pub fn new() -> ObserverConfig {
            let host = std::env::var("OBSERVER_HOST").expect("OBSERVER_HOST must be set");
            let port = std::env::var("OBSERVER_PORT").expect("OBSERVER_PORT must be set");
            ObserverConfig { host, port }
        }

        pub fn to_string(&self) -> String {
            format!("http://{}:{}", self.host, self.port)
        }
    }
}

pub fn detect_log_type(log_type: &str) -> LogType {
    match log_type {
        "event" => Some(LogType::Event),
        "runtime" => Some(LogType::Runtime),
        "input" => Some(LogType::Input),
        "output" => Some(LogType::Output),
        "feedback" => Some(LogType::Feedback),
        "metadata" => Some(LogType::Metadata),
        _ => None
    }.unwrap_or_else(|| panic!("Unknown log type {}", log_type))
}

pub fn detect_tier(tier: &str) -> Tier {
    match tier {
        "system" => Some(Tier::System),
        "subsystem" => Some(Tier::Subsystem),
        "component" => Some(Tier::Component),
        "subcomponent" => Some(Tier::Subcomponent),
        _ => None
    }.unwrap_or_else(|| panic!("Unknown tier {}", tier))
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct ProtoRecord {
    tier: Tier,
    log_type: LogType,
    parent_id: String,
    id: String,
    name: Option<String>,
    parameters: Option<JsonSerializable>,
    version: Option<String>,
    environment: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    error_type: Option<String>,
    error_content: Option<String>,
    field_name: Option<String>,
    field_value: Option<JsonSerializable>,
}


impl ProtoRecord {
    pub fn new(tier: Tier, log_type: LogType, parent_id: String, id: String) -> Self {
        ProtoRecord {
            tier: tier,
            log_type: log_type,
            parent_id,
            id,
            name: None,
            parameters: None,
            version: None,
            environment: None,
            start_time: None,
            end_time: None,
            error_type: None,
            error_content: None,
            field_name: None,
            field_value: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn parameters(mut self, parameters: Option<JsonSerializable>) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    pub fn environment(mut self, environment: String) -> Self {
        self.environment = Some(environment);
        self
    }

    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn error_type(mut self, error_type: String) -> Self {
        self.error_type = Some(error_type);
        self
    }

    pub fn error_content(mut self, error_content: String) -> Self {
        self.error_content = Some(error_content);
        self
    }

    pub fn field_name(mut self, field_name: String) -> Self {
        self.field_name = Some(field_name);
        self
    }

    pub fn field_value(mut self, field_value: JsonSerializable) -> Self {
        self.field_value = Some(field_value);
        self
    }

    pub fn proto(self) -> Record {
        Record {
            tier: self.tier.into(),
            log_type: self.log_type.into(),
            parent_id: self.parent_id,
            id: self.id,
            name: self.name,
            parameters: match self.parameters {
                None => None,
                Some(value) => json_serializable_to_value(&Some(value))
            },
            version: self.version,
            environment: self.environment,
            start_time: self.start_time,
            end_time: self.end_time,
            error_type: self.error_type,
            error_content: self.error_content,
            field_name: self.field_name,
            field_value: match self.field_value {
                None => None,
                Some(value) => json_serializable_to_value(&Some(value))
            },
        }
    }
}

#[pyclass]
pub struct RecordBuilder;

#[pymethods]
impl RecordBuilder {
    #[new]
    pub fn new() -> RecordBuilder {
        RecordBuilder { }
    }

    #[pyo3(signature = (tier, parent_id, id, name, parameters=None, version=None, environment=None))]
    #[staticmethod]
    pub fn event(tier: &str, parent_id: String, id: String, name: String, parameters: Option<JsonSerializable>, version: Option<String>, environment: Option<String>) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Event,
            parent_id,
            id
        )
        .name(name)
        .parameters(parameters)
        .version(version.unwrap_or_default())
        .environment(environment.unwrap_or_default())
    }

    #[pyo3(signature = (tier, parent_id, id, start_time, end_time, error_type=None, error_content=None))]
    #[staticmethod]
    pub fn runtime(tier: &str, parent_id: String, id: String, start_time: String, end_time: String, error_type: Option<String>, error_content: Option<String>) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Runtime,
            parent_id,
            id,
        )
        .start_time(start_time)
        .end_time(end_time)
        .error_type(error_type.unwrap_or_default())
        .error_content(error_content.unwrap_or_default())
    }

    #[pyo3(signature = (tier, parent_id, id, field_name, field_value))]
    #[staticmethod]
    pub fn input(tier: &str, parent_id: String, id: String, field_name: String, field_value: JsonSerializable) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Input,
            parent_id,
            id,
        )
        .field_name(field_name)
        .field_value(field_value)
    }

    #[staticmethod]
    pub fn output(tier: &str, parent_id: String, id: String, field_name: String, field_value: JsonSerializable) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Output,
            parent_id,
            id,
        )
        .field_name(field_name)
        .field_value(field_value)
    }

    #[pyo3(signature = (tier, parent_id, id, field_name, field_value))]
    #[staticmethod]
    pub fn feedback(tier: &str, parent_id: String, id: String, field_name: String, field_value: JsonSerializable) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Feedback,
            parent_id,
            id,
        )
        .field_name(field_name)
        .field_value(field_value)
    }

    #[staticmethod]
    pub fn metadata(tier: &str, parent_id: String, id: String, field_name: String, field_value: JsonSerializable) -> ProtoRecord {
        ProtoRecord::new(
            detect_tier(tier),
            LogType::Metadata,
            parent_id,
            id,
        )
        .field_name(field_name)
        .field_value(field_value)
    }
}

#[pyclass]
pub struct BlockingObserverClient {
    client: ObserverClient<Channel>,
    rt: tokio::runtime::Runtime
}

impl BlockingObserverClient {
    pub fn connect(config: client_config::ObserverConfig) -> Result<BlockingObserverClient, Box<dyn std::error::Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = rt.block_on(ObserverClient::connect(config.to_string()))?;

        Ok(BlockingObserverClient { client, rt })
    }

    pub fn publish_request(&mut self, records: Vec<Record>) -> Result<PublishResponse, Box<dyn std::error::Error>> {
        self.rt.block_on(
            async {
                let publish_request = tonic::Request::new(
                    PublishRequest {
                        records: records
                    }
                );
                let response = self.client.publish(publish_request).await?;

                Ok(response.into_inner())
            }
        )
    }
}

#[pymethods]
impl BlockingObserverClient {
    #[new]
    pub fn new() -> Self {
        let config = client_config::ObserverConfig::new();
        BlockingObserverClient::connect(config).unwrap()
    }

    pub fn publish_records(&mut self, records: Vec<ProtoRecord>) -> bool {
        let records = records.iter().map(|r| r.clone().proto()).collect();

        let success = match self.publish_request(records) {
            Ok(_) => true,
            Err(e) => {
                println!("Error publishing records: {}", e);
                false
            }
        };

        success
    }
}

#[derive(FromPyObject, Clone, Debug)]
pub enum JsonSerializable {
    String(String),
    Int(isize),
    Float(f64),
    Bool(bool),
    Dict(BTreeMap<String, Option<JsonSerializable>>),
    List(Vec<Option<JsonSerializable>>)
}

fn json_serializable_to_value(json: &Option<JsonSerializable>) -> Option<Value> {
    match json {
        Some(JsonSerializable::String(s)) => Some(Value {
            kind: Some(Kind::StringValue(s.clone())),
        }),
        Some(JsonSerializable::Int(i)) => Some(Value {
            kind: Some(Kind::NumberValue(*i as f64)),
        }),
        Some(JsonSerializable::Float(f)) => Some(Value {
            kind: Some(Kind::NumberValue(*f)),
        }),
        Some(JsonSerializable::Bool(b)) => Some(Value {
            kind: Some(Kind::BoolValue(*b)),
        }),
        Some(JsonSerializable::Dict(d)) => {
            let mut fields = BTreeMap::new();
            for (k, v) in d {
                if let Some(value) = json_serializable_to_value(v) {
                    fields.insert(k.clone(), value);
                }
            }
            Some(Value {
                kind: Some(Kind::StructValue(Struct { fields })),
            })
        }
        Some(JsonSerializable::List(l)) => {
            let values: Vec<Value> = l
                .iter()
                .filter_map(|v| json_serializable_to_value(v))
                .collect();
            Some(Value {
                kind: Some(Kind::ListValue(ListValue { values })),
            })
        }
        None => None,
    }
}
