// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        #[max_length = 15]
        username -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        #[max_length = 15]
        password -> Varchar,
        salt -> Varchar
    }
}
