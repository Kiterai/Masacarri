use diesel::Queryable;
use diesel::QueryableByName;
use serde::Serialize;
use diesel::sql_types::*;

use crate::schema::comments;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
    pub flags: i32,
}

#[derive(Queryable, Serialize)]
pub struct Page {
    pub id: uuid::Uuid,
    pub title: String,
    pub page_url: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Comment {
    pub id: uuid::Uuid,
    pub page_id: uuid::Uuid,
    pub reply_to: Option<uuid::Uuid>,
    pub ip_addr: ipnetwork::IpNetwork,
    pub display_name: String,
    pub site_url: Option<String>,
    pub mail_addr: Option<String>,
    pub content: String,
    pub delete_key: String,
    pub flags: i32,
    pub created_time: chrono::DateTime<chrono::Utc>,
}


#[derive(Queryable, QueryableByName)]
#[table_name = "comments"]
pub struct CommentWithReplies {
    pub id: uuid::Uuid,
    pub page_id: uuid::Uuid,
    pub reply_to: Option<uuid::Uuid>,
    pub ip_addr: ipnetwork::IpNetwork,
    pub display_name: String,
    pub site_url: Option<String>,
    pub mail_addr: Option<String>,
    pub content: String,
    pub delete_key: String,
    pub flags: i32,
    pub created_time: chrono::DateTime<chrono::Utc>,
    #[sql_type = "BigInt"]
    pub count_replies: i64,
}
