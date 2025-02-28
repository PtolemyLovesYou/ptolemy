mod auth;
mod enums;
mod event;
mod id;

mod json {
    use crate::error::ParseError;
    use prost_types::{value::Kind, ListValue, Struct};

    type ProtoValue = prost_types::Value;
    type JsonValue = serde_json::Value;

    #[derive(Clone, Debug)]
    pub struct JSON(pub JsonValue);

    impl serde::Serialize for JSON {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.0.serialize(serializer)
        }
    }

    impl<'de> serde::Deserialize<'de> for JSON {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(JSON(serde_json::Value::deserialize(deserializer)?))
        }
    }

    impl From<JSON> for ProtoValue {
        fn from(val: JSON) -> Self {
            serde_value_to_protobuf(&val.0)
        }
    }

    impl From<JSON> for JsonValue {
        fn from(val: JSON) -> Self {
            val.0
        }
    }

    impl TryFrom<ProtoValue> for JSON {
        type Error = ParseError;

        fn try_from(value: ProtoValue) -> Result<Self, Self::Error> {
            match protobuf_to_serde_value(&value) {
                Some(s) => Ok(JSON(s)),
                None => Err(ParseError::UnexpectedNull),
            }
        }
    }

    fn serde_value_to_protobuf(value: &JsonValue) -> ProtoValue {
        match value {
            JsonValue::Null => ProtoValue {
                kind: Some(Kind::NullValue(0)),
            },
            JsonValue::Number(n) => ProtoValue {
                kind: Some(Kind::NumberValue(n.as_f64().unwrap())),
            },
            JsonValue::String(s) => ProtoValue {
                kind: Some(Kind::StringValue(s.clone())),
            },
            JsonValue::Bool(b) => ProtoValue {
                kind: Some(Kind::BoolValue(*b)),
            },
            JsonValue::Array(l) => ProtoValue {
                kind: Some(Kind::ListValue(ListValue {
                    values: l.iter().map(serde_value_to_protobuf).collect(),
                })),
            },
            JsonValue::Object(m) => ProtoValue {
                kind: Some(Kind::StructValue(Struct {
                    fields: m
                        .iter()
                        .map(|(k, v)| (k.clone(), serde_value_to_protobuf(v)))
                        .collect(),
                })),
            },
        }
    }

    fn protobuf_to_serde_value(value: &ProtoValue) -> Option<JsonValue> {
        match &value.kind {
            Some(Kind::NullValue(_)) => Some(JsonValue::Null),
            Some(Kind::NumberValue(v)) => {
                Some(JsonValue::Number(serde_json::Number::from_f64(*v)?))
            }
            Some(Kind::StringValue(v)) => Some(JsonValue::String(v.clone())),
            Some(Kind::BoolValue(v)) => Some(JsonValue::Bool(v.clone())),
            Some(Kind::StructValue(v)) => {
                let mut map = serde_json::Map::new();
                for (field, value) in &v.fields {
                    let val = protobuf_to_serde_value(value).unwrap();
                    map.insert(field.clone(), val);
                }
                Some(JsonValue::Object(map))
            }
            Some(Kind::ListValue(v)) => {
                let mut list = Vec::new();
                for value in &v.values {
                    list.push(protobuf_to_serde_value(value).unwrap());
                }
                Some(JsonValue::Array(list))
            }
            None => None,
        }
    }
}

pub use auth::{ServiceApiKey, User, UserApiKey, Workspace, WorkspaceUser};
pub use enums::{ApiKeyPermission, Tier, UserStatus, WorkspaceRole};
pub use event::{
    Proto, ProtoEvent, ProtoFeedback, ProtoInput, ProtoMetadata, ProtoOutput, ProtoRecord,
    ProtoRuntime,
};
pub use id::Id;
pub use json::JSON;
