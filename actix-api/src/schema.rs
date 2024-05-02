// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}
