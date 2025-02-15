pub mod audit;
pub mod auth;
pub mod enum_utils;
pub mod middleware;
pub mod query;
pub mod records;

pub use self::audit::*;
pub use self::auth::*;

pub mod prelude {
    pub trait HasId {
        fn id(&self) -> uuid::Uuid;
    }

    impl HasId for uuid::Uuid {
        fn id(&self) -> uuid::Uuid {
            self.clone()
        }
    }

    #[macro_export]
    macro_rules! impl_has_id {
        ($t:ty) => {
            impl crate::models::prelude::HasId for $t {
                fn id(&self) -> uuid::Uuid {
                    self.id.clone()
                }
            }
        };
    }
}
