#[macro_export]
macro_rules! define_enum {
    ($name:ident, $type:tt, [$($variant:ident),+]) => {
        #[derive(
            Clone, Debug, PartialEq, FromSqlRow, AsExpression, Eq, GraphQLEnum,
        )]
        #[diesel(sql_type = $type)]
        pub enum $name {
            $($variant),+
        }

        impl ToSql<$type, Pg> for $name {
            fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
                use heck::ToSnakeCase;

                match *self {
                    $(
                        $name::$variant => out.write_all(stringify!($variant).to_snake_case().as_bytes())?,
                    )+
                }

                Ok(IsNull::No)
            }
        }

        impl $name {
            pub fn to_string(&self) -> String {
                use heck::ToShoutySnakeCase;
                match self {
                    $(
                        $name::$variant => stringify!($variant).to_shouty_snake_case(),
                    )+
                }
            }
        }

        impl FromSql<$type, Pg> for $name {
            fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
                use heck::ToSnakeCase;

                let input = bytes.as_bytes();
                $(
                    if input == stringify!($variant).to_snake_case().as_bytes() {
                        return Ok($name::$variant);
                    }
                )+

                Err(format!("Unrecognized enum variant: {}", String::from_utf8_lossy(bytes.as_bytes())).into())
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Ok(self.to_string().serialize(serializer)?)
            }
        }

        impl <'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use heck::ToPascalCase;

                let s = String::deserialize(deserializer)?;
                match s.to_pascal_case().as_str() {
                    $(
                        stringify!($variant) => Ok($name::$variant),
                    )+
                    _ => Err(serde::de::Error::custom(format!("Unrecognized enum variant: {}", s))),
                }
            }
        }
    };

    ($name:ident, $type:tt, [$($variant:ident),+], WithConversion) => {
        define_enum!($name, $type, [$($variant),+]);

        impl Into<ptolemy::models::enums::$type> for $name {
            fn into(self) -> ptolemy::models::enums::$type {
                match self {
                    $(
                        $name::$variant => ptolemy::models::enums::$type::$variant,
                    )+
                }
            }
        }

        impl From<ptolemy::models::enums::$type> for $name {
            fn from(value: ptolemy::models::enums::$type) -> Self {
                match value {
                    $(
                        ptolemy::models::enums::$type::$variant => $name::$variant,
                    )+
                }
            }
        }
    }
}
