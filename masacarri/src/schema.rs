table! {
    comments (id) {
        id -> Uuid,
        page_id -> Uuid,
        reply_to -> Nullable<Uuid>,
        ip_addr -> Inet,
        display_name -> Varchar,
        site_url -> Nullable<Varchar>,
        mail_addr -> Nullable<Varchar>,
        content -> Varchar,
        delete_key -> Varchar,
        flags -> Int4,
        created_time -> Timestamptz,
    }
}

table! {
    pages (id) {
        id -> Uuid,
        title -> Varchar,
        page_url -> Varchar,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password_hash -> Varchar,
        flags -> Int4,
    }
}

joinable!(comments -> pages (page_id));

allow_tables_to_appear_in_same_query!(
    comments,
    pages,
    users,
);
