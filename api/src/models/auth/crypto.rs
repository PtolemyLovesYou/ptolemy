use diesel::{define_sql_function, sql_types::Text};

define_sql_function! {
    fn crypt(password: Text, salt: Text) -> Text;
}

// For password verification
define_sql_function! {
    fn gen_salt(text: Text) -> Text;
}
