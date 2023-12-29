table! {
    session (id) {
        id -> Int4,
        user_id -> Int4,
        valid_until -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    workspace (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        type_id -> Int4,
        styles -> Nullable<Text>,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        content -> Nullable<Text>,
    }
}

table! {
    workspace_element (id) {
        id -> Int4,
        parent_id -> Int4,
        child_id -> Int4,
        user_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    workspace_type (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
    }
}

joinable!(workspace -> workspace_type (type_id));

allow_tables_to_appear_in_same_query!(
    session,
    user,
    workspace,
    workspace_element,
    workspace_type,
);
