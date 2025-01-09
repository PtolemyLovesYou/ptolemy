use heck::{ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
pub use serde::{Deserialize, Serialize, Serializer, Deserializer};
pub use crate::serialize_enum;

#[derive(Debug)]
pub enum CasingStyle {
    ShoutySnakeCase,
    SnakeCase,
    LowerCamelCase,
    PascalCase,
}

pub trait SerializableEnum<'de>: Into<String> + TryFrom<String> + Serialize + Deserialize<'de> {}

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
        impl Into<String> for $enum_name {
            fn into(self) -> String {
                match self {
                    $(
                        Self::$variant => CasingStyle::$casing.format(stringify!($variant)),
                    )+
                }
            }
        }

        impl TryFrom<String> for $enum_name {
            type Error = crate::error::ParseError;

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
        MyEnumOne,
        MyEnumTwo,
        MyEnumThree,
    }

    serialize_enum!(MyEnum, ShoutySnakeCase, [MyEnumOne, MyEnumTwo, MyEnumThree]);

    #[test]
    fn test_serialize_enum() {
        // serialize enum with serde
        let serialized_val = serde_json::to_string(&MyEnum::MyEnumOne.clone()).unwrap();
        assert_eq!(serialized_val, "\"MY_ENUM_ONE\"");
    }

    #[test]
    fn test_deserialize_enum() {
        // deserialize enum with serde
        let deserialized_val: MyEnum = serde_json::from_str("\"MY_ENUM_ONE\"").unwrap();
        assert_eq!(deserialized_val, MyEnum::MyEnumOne);
    }

    #[test]
    fn test_bad_enum() {
        let bad_val: Result<MyEnum, serde_json::Error> = serde_json::from_str("\"BadValue\"");
        assert!(bad_val.is_err());
    }
}
