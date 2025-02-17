pub use crate::serialize_enum;
use heck::{ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
pub use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub enum CasingStyle {
    ShoutySnakeCase,
    SnakeCase,
    LowerCamelCase,
    PascalCase,
}

pub trait SerializableEnum<'de>:
    Into<String> + TryFrom<String> + Serialize + Deserialize<'de>
{
}

impl CasingStyle {
    pub fn format(&self, variant: &str) -> String {
        match self {
            CasingStyle::ShoutySnakeCase => variant.to_shouty_snake_case(),
            CasingStyle::SnakeCase => variant.to_snake_case(),
            CasingStyle::LowerCamelCase => variant.to_lower_camel_case(),
            CasingStyle::PascalCase => variant.to_pascal_case(),
        }
    }
}

#[macro_export]
macro_rules! serialize_enum {
    ($enum_name:ident, $casing:ident, [$($variant:ident),+ $(,)?]) => {
        impl From<$enum_name> for String {
            fn from(value: $enum_name) -> Self {
                match value {
                    $(
                        $enum_name::$variant => CasingStyle::$casing.format(stringify!($variant)),
                    )+
                }
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = $crate::error::ParseError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                match value.as_str() {
                    $(
                        v if v == CasingStyle::$casing.format(stringify!($variant)) => Ok(Self::$variant),
                    )+
                    _ => Err(Self::Error::BadEnum(format!("Invalid enum value: {}", value)))
                }
            }
        }

        impl Serialize for $enum_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let s: String = self.clone().into();

                serializer.serialize_str(&s)
            }
        }

        impl<'de> Deserialize<'de> for $enum_name {
            fn deserialize<D>(deserializer: D) -> Result<$enum_name, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                $enum_name::try_from(s).map_err(|e| serde::de::Error::custom(format!("Invalid enum value: {:?}", e)))
            }
        }

        impl<'de> SerializableEnum<'de> for $enum_name {}
    }
}

mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum MyEnum {
        OneFoo,
        TwoBar,
        ThreeBaz,
    }

    serialize_enum!(MyEnum, ShoutySnakeCase, [OneFoo, TwoBar, ThreeBaz]);

    #[test]
    fn test_serialize_enum() {
        // serialize enum with serde
        let serialized_val = serde_json::to_string(&MyEnum::OneFoo.clone()).unwrap();
        assert_eq!(serialized_val, "\"ONE_FOO\"");
    }

    #[test]
    fn test_deserialize_enum() {
        // deserialize enum with serde
        let deserialized_val: MyEnum = serde_json::from_str("\"ONE_FOO\"").unwrap();
        assert_eq!(deserialized_val, MyEnum::OneFoo);
    }

    #[test]
    fn test_bad_enum() {
        let bad_val: Result<MyEnum, serde_json::Error> = serde_json::from_str("\"BadValue\"");
        assert!(bad_val.is_err());
    }
}
