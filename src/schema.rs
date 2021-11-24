table! {
    posts (id) {
        id -> Nullable<Bigint>,
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Nullable<Bigint>,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
