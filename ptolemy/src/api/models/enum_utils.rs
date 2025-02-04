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

        impl Into<crate::models::enums::$type> for $name {
            fn into(self) -> crate::models::enums::$type {
                match self {
                    $(
                        $name::$variant => crate::models::enums::$type::$variant,
                    )+
                }
            }
        }

        impl From<crate::models::enums::$type> for $name {
            fn from(value: crate::models::enums::$type) -> Self {
                match value {
                    $(
                        crate::models::enums::$type::$variant => $name::$variant,
                    )+
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s: crate::models::enums::$type = self.clone().into();
                s.serialize(serializer)
            }
        }

        impl <'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s: crate::models::enums::$type = serde::Deserialize::deserialize(deserializer)?;
                Ok(s.into())
            }
        }
    }
}
