// @generated automatically by Diesel CLI.

diesel::table! {
    user (user_id) {
        #[max_length = 36]
        user_id -> Varchar,
        #[max_length = 50]
        nickname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 50]
        phone_number -> Varchar,
        password_hash -> Text,
        last_login_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
