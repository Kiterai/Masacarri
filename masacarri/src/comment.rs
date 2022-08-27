use crate::bgtask::BgTaskManager;
use crate::db::Pool;
use crate::error::{AppError, AppResult};
use crate::mail::MailNotifyTask;
use crate::models::{Comment, CommentWithReplies, CountResult};
use crate::schema::comments;
use crate::schema::comments::dsl::*;
use crate::utils::empty_to_none;
use actix_identity::Identity;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use diesel::pg::types::sql_types;
use diesel::sql_types::BigInt;
use diesel::{prelude::*, sql_query};
use serde::{Deserialize, Serialize};
use serde_json::json;
use static_assertions::const_assert;

const MARK_AS_SPAM_FRAG_BIT: i32 = 1;

const DEFAULT_COMMENTS_PER_PAGE: u32 = 10;
const DEFAULT_PAGE_INDEX: u32 = 1;
const MAX_COMMENTS_PER_PAGE: u32 = 256;

const_assert!(DEFAULT_COMMENTS_PER_PAGE <= MAX_COMMENTS_PER_PAGE);
const_assert!(DEFAULT_PAGE_INDEX == 1);

#[derive(Deserialize)]
pub struct NewCommentRequest {
    reply_to: Option<uuid::Uuid>,
    display_name: String,
    site_url: Option<String>,
    mail_addr: Option<String>,
    content: String,
    delete_key: Option<String>,
}

#[derive(Deserialize)]
pub struct NewCommentRequestPath {
    page: uuid::Uuid,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    id: uuid::Uuid,
    page_id: uuid::Uuid,
    reply_to: Option<uuid::Uuid>,
    ip_addr: ipnetwork::IpNetwork,
    display_name: String,
    site_url: Option<String>,
    mail_addr: Option<String>,
    content: String,
    delete_key: String,
    flags: i32,
    created_time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct GetCommentsRequestPath {
    page: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct GetCommentsRequestQuery {
    num: Option<u32>,
    index: Option<u32>,
    replyto: Option<uuid::Uuid>,
    contextof: Option<uuid::Uuid>,
}

#[derive(Deserialize)]
pub struct GetCommentRequestPath {
    page: uuid::Uuid,
    comment: uuid::Uuid,
}

#[derive(Queryable, Serialize)]
pub struct GetCommentResponse {
    id: uuid::Uuid,
    page_id: uuid::Uuid,
    reply_to: Option<uuid::Uuid>,
    display_name: String,
    site_url: Option<String>,
    content: String,
    count_replies: Option<i64>,
    created_time: DateTime<Utc>,
}

impl From<CommentWithReplies> for GetCommentResponse {
    fn from(comment: CommentWithReplies) -> Self {
        let CommentWithReplies {
            id: r_id,
            page_id: r_page_id,
            reply_to: r_reply_to,
            display_name: r_display_name,
            site_url: r_site_url,
            content: r_content,
            ip_addr: _,
            mail_addr: _,
            delete_key: _,
            flags: r_flags,
            count_replies: r_count_replies,
            created_time: r_created_time,
        } = comment;

        let is_spam = (r_flags & MARK_AS_SPAM_FRAG_BIT) == MARK_AS_SPAM_FRAG_BIT;

        GetCommentResponse {
            id: r_id,
            page_id: r_page_id,
            reply_to: r_reply_to,
            display_name: r_display_name,
            site_url: r_site_url,
            content: match is_spam {
                true => "(This comment is marked as spam.)".to_string(),
                _ => r_content,
            },
            count_replies: Some(r_count_replies),
            created_time: r_created_time,
        }
    }
}

impl From<Comment> for GetCommentResponse {
    fn from(comment: Comment) -> Self {
        let Comment {
            id: r_id,
            page_id: r_page_id,
            reply_to: r_reply_to,
            display_name: r_display_name,
            site_url: r_site_url,
            content: r_content,
            ip_addr: _,
            mail_addr: _,
            delete_key: _,
            flags: r_flags,
            created_time: r_created_time,
        } = comment;

        let is_spam = (r_flags & MARK_AS_SPAM_FRAG_BIT) == MARK_AS_SPAM_FRAG_BIT;

        GetCommentResponse {
            id: r_id,
            page_id: r_page_id,
            reply_to: r_reply_to,
            display_name: r_display_name,
            site_url: r_site_url,
            content: match is_spam {
                true => "(This comment is marked as spam.)".to_string(),
                _ => r_content,
            },
            count_replies: None,
            created_time: r_created_time,
        }
    }
}

pub async fn add_comment(
    db: web::Data<Pool>,
    path_param: web::Path<NewCommentRequestPath>,
    req: HttpRequest,
    new_comment: web::Json<NewCommentRequest>,
    bgtask_manager: web::Data<BgTaskManager>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let NewCommentRequest {
        reply_to: r_reply_to,
        display_name: r_display_name,
        site_url: r_site_url,
        mail_addr: r_mail_addr,
        content: r_content,
        delete_key: r_delete_key,
    } = new_comment.into_inner();

    if let Some(reply_to_id) = r_reply_to {
        let reply_to_page_id = comments
            .select(page_id)
            .filter(id.eq(reply_to_id))
            .first::<uuid::Uuid>(&conn)?;

        if reply_to_page_id != path_param.page {
            return Err(AppError::PublishableErr(
                "You replied to an invalid comment.".to_string(),
            ));
        }
    }

    let new_id = uuid::Uuid::new_v4();

    let ipaddr = req.peer_addr().ok_or(AppError::UnspecifiedErr)?.ip();

    let ipaddr = ipnetwork::IpNetwork::new(
        ipaddr,
        if ipaddr.is_ipv4() {
            32
        } else if ipaddr.is_ipv6() {
            128
        } else {
            0
        },
    )?;

    let r_delete_key = match r_delete_key {
        Some(val) => bcrypt::hash(val, 16).unwrap(),
        None => "-".to_string(),
    };

    if r_display_name.is_empty() {
        return Err(AppError::PublishableErr(
            "Display name is required.".to_owned(),
        ));
    }
    if r_content.is_empty() {
        return Err(AppError::PublishableErr(
            "Comment text is required.".to_owned(),
        ));
    }

    let res = diesel::insert_into(comments)
        .values(NewComment {
            id: new_id,
            page_id: path_param.page,
            reply_to: r_reply_to,
            ip_addr: ipaddr,
            display_name: r_display_name,
            site_url: empty_to_none(r_site_url),
            mail_addr: empty_to_none(r_mail_addr),
            content: r_content,
            delete_key: r_delete_key,
            flags: 0,
            created_time: Utc::now(),
        })
        .execute(&conn);
    if let Err(_) = res {
        return Err(AppError::PublishableErr(
            "Failed to post new comment.".to_owned(),
        ));
    }

    let mut result = comments.filter(id.eq(new_id)).load::<Comment>(&conn)?;
    if result.len() != 1 {
        return Err(AppError::UnspecifiedErr);
    }

    let comment_new = result.pop().ok_or(AppError::UnspecifiedErr)?;
    if let Some(id_replyto) = comment_new.reply_to {
        let comment_new = comment_new.clone();

        bgtask_manager.do_send(MailNotifyTask{
            id_replyto: id_replyto,
            conn: conn,
            comment_new: comment_new,
        });
    }

    Ok(HttpResponse::Created().json(GetCommentResponse::from(comment_new)))
}

pub async fn get_comments(
    db: web::Data<Pool>,
    path_param: web::Path<GetCommentsRequestPath>,
    query_param: web::Query<GetCommentsRequestQuery>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let comments_per_page = query_param.num.unwrap_or(DEFAULT_COMMENTS_PER_PAGE);
    let comments_page_index = query_param.index.unwrap_or(DEFAULT_PAGE_INDEX);

    if comments_page_index < 1 {
        return Err(AppError::PublishableErr("invalid page index".to_string()));
    }

    let comments_page_index = comments_page_index - 1;

    if 0 >= comments_per_page || comments_per_page > MAX_COMMENTS_PER_PAGE {
        return Err(AppError::PublishableErr(format!(
            "Comments per page is limited up to {}.",
            MAX_COMMENTS_PER_PAGE
        )));
    }

    let result = match (query_param.replyto, query_param.contextof) {
        (Some(_), Some(_)) => {
            return Err(AppError::PublishableErr(format!(
                "'replyto' and 'contextof' are not allowed to use simultaneously.",
            )));
        },
        (Some(reply_to_id), None) => sql_query(
            r#"
                select comments.*, count(child_comments.id) as count_replies
                from comments
                left join comments as child_comments 
                on comments.id = child_comments.reply_to
                where comments.reply_to = $1
                group by comments.id
                order by created_time
                offset $2
                limit $3;
            "#,
        )
        .bind::<sql_types::Uuid, _>(reply_to_id)
        .bind::<BigInt, i64>((comments_per_page * comments_page_index).into())
        .bind::<BigInt, i64>((comments_per_page).into())
        .load::<CommentWithReplies>(&conn)?,
        (None, Some(target_comment_id)) => sql_query(
            r#"
            with recursive tree as (
                select comments.*
                from comments
                where comments.id = $1
                union all
                    select comments.*
                    from tree, comments
                    where tree.reply_to = comments.id
            )
            select * from (
                select distinct on (tree.id) tree.*, count(comments.id) over (partition by tree.id) as count_replies
                from tree
                left join comments
                on tree.id = comments.reply_to
                order by tree.id
            ) as context
            order by created_time
            offset $2
            limit $3;
            "#,
        )
        .bind::<sql_types::Uuid, _>(target_comment_id)
        .bind::<BigInt, i64>((comments_per_page * comments_page_index).into())
        .bind::<BigInt, i64>((comments_per_page).into())
        .load::<CommentWithReplies>(&conn)?,
        (None, None) => sql_query(
            r#"
                select comments.*, count(child_comments.id) as count_replies
                from comments
                left join comments as child_comments
                on comments.id = child_comments.reply_to
                where comments.page_id = $1
                group by comments.id
                order by created_time
                offset $2
                limit $3;
            "#,
        )
        .bind::<sql_types::Uuid, _>(path_param.page)
        .bind::<BigInt, i64>((comments_per_page * comments_page_index).into())
        .bind::<BigInt, i64>((comments_per_page).into())
        .load::<CommentWithReplies>(&conn)?,
    };

    let showing_comments: Vec<_> = result
        .into_iter()
        .map(move |comment| GetCommentResponse::from(comment))
        .collect();

    Ok(HttpResponse::Ok().json(showing_comments))
}

pub async fn get_comment(
    db: web::Data<Pool>,
    path_param: web::Path<GetCommentRequestPath>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let result = sql_query(
        r#"
                select comments.*, count(child_comments.id) as count_replies
                from comments
                left join comments as child_comments
                on comments.id = child_comments.reply_to
                where comments.id = $1 and comments.page_id = $2
                group by comments.id
            "#,
    )
    .bind::<sql_types::Uuid, _>(path_param.comment)
    .bind::<sql_types::Uuid, _>(path_param.page)
    .load::<CommentWithReplies>(&conn)?;

    let result = result.into_iter().next();

    match result {
        Some(comment) => Ok(HttpResponse::Ok().json(GetCommentResponse::from(comment))),
        None => Ok(HttpResponse::NotFound().json(json!({
            "message": "comment not found",
        }))),
    }
}

pub async fn mark_comment(_: Identity) -> AppResult<impl Responder> {
    Ok(HttpResponse::NoContent())
}

pub async fn get_comment_count(
    db: web::Data<Pool>,
    path_param: web::Path<GetCommentsRequestPath>,
    query_param: web::Query<GetCommentsRequestQuery>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let result: i64 = match (query_param.replyto, query_param.contextof) {
        (None, None) => comments
            .filter(page_id.eq(path_param.page))
            .count()
            .get_result(&conn)?,
        (None, Some(contextof_id)) => {
            sql_query(
                r#"
            with recursive tree as (
                select comments.reply_to
                from comments
                where comments.id = $1
                union all
                    select comments.reply_to
                    from tree, comments
                    where tree.reply_to = comments.id
            )
            select count(*) from tree
            "#,
            )
            .bind::<sql_types::Uuid, _>(contextof_id)
            .get_result::<CountResult>(&conn)?
            .count
        }
        (Some(reply_to_id), None) => comments
            .filter(reply_to.eq(reply_to_id))
            .count()
            .get_result(&conn)?,
        (Some(_), Some(_)) => {
            return Err(AppError::PublishableErr(format!(
                "'replyto' and 'contextof' are not allowed to use simultaneously.",
            )));
        }
    };

    Ok(HttpResponse::Ok().json(json!({
        "count": result,
    })))
}
