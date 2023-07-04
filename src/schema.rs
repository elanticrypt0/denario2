// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}

diesel::table! {
    credits (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        comment -> Nullable<Text>,
        amount -> Float8,
        payments -> Int4,
        started_at -> Date,
        finish_at -> Date,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
    }
}

diesel::table! {
    records (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        amount -> Float8,
        #[max_length = 3]
        amount_io -> Varchar,
        comment -> Nullable<Text>,
        record_date -> Date,
        category_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_deleted -> Bool,
        is_mutable -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    credits,
    records,
);
