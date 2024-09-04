use diesel::table;
use diesel::allow_tables_to_appear_in_same_query;

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    blogs (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        author_id -> Uuid,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    comments (id) {
        id -> Uuid,
        blog_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        parent_comment_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

table! {
    likes (id) {
        id -> Uuid,
        blog_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    blogs,
    comments,
    likes,
);