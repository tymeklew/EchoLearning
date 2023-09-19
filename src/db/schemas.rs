use diesel::{allow_tables_to_appear_in_same_query, joinable, table};

table! {
    users {
        id -> Varchar,
        email -> Text,
        name -> Text,
        password -> VarChar,
        created_at -> Datetime,
    }
}

table! {
    sessions {
        id -> VarChar,
        user_id -> VarChar,
        created_at -> Datetime,
        expires_at -> Datetime,
    }
}
table! {
    resets {
        id -> VarChar,
        user_id -> VarChar,
        secret -> VarChar
    }
}

joinable!(sessions -> users (user_id));
allow_tables_to_appear_in_same_query!(users, sessions, resets);
