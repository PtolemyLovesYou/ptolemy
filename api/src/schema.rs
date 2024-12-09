// @generated automatically by Diesel CLI.

diesel::table! {
    workspace (id) {
        id -> Uuid,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
