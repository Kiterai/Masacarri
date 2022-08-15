use crate::db::Pool;
use crate::error::{AppError, AppResult};
use crate::models::Page;
use crate::schema::pages;
use crate::schema::pages::dsl::*;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewPageRequest {
    title: String,
    page_url: String,
    published: bool,
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    id: uuid::Uuid,
    title: String,
    page_url: String,
    published: bool,
}

#[derive(Deserialize)]
pub struct ModifyPageRequest {
    title: String,
    page_url: String,
    published: bool,
}

#[derive(Deserialize)]
pub struct ModifyPageRequestPath {
    page: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct DeletePageRequestPath {
    page: uuid::Uuid,
}

pub async fn get_page_all(_: Identity, db: web::Data<Pool>) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let result = pages.load::<Page>(&conn)?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn add_page(
    _: Identity,
    db: web::Data<Pool>,
    new_page: web::Json<NewPageRequest>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let NewPageRequest {
        title: r_title,
        page_url: r_page_url,
        published: r_published,
    } = new_page.into_inner();

    let new_id = uuid::Uuid::new_v4();

    let res = diesel::insert_into(pages)
        .values(NewPage {
            id: new_id,
            title: r_title,
            page_url: r_page_url,
            published: r_published,
        })
        .execute(&conn);
    if let Err(_) = res {
        return Err(AppError::PublishableErr(
            "Failed to insert new page.".to_owned(),
        ));
    }

    let result = pages.filter(id.eq(new_id)).load::<Page>(&conn)?;
    if result.len() != 1 {
        return Err(AppError::UnspecifiedErr);
    }

    Ok(HttpResponse::Created().json(result.first().ok_or(AppError::UnspecifiedErr)?))
}

pub async fn modify_page(
    _: Identity,
    db: web::Data<Pool>,
    path_param: web::Path<ModifyPageRequestPath>,
    updated_page: web::Json<ModifyPageRequest>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    diesel::update(pages.filter(id.eq(path_param.page)))
        .set((
            title.eq(&updated_page.title),
            page_url.eq(&updated_page.page_url),
            published.eq(&updated_page.published),
        ))
        .execute(&conn)?;
    Ok(HttpResponse::NoContent())
}

pub async fn delete_page(
    _: Identity,
    db: web::Data<Pool>,
    path_param: web::Path<DeletePageRequestPath>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    diesel::delete(pages.filter(id.eq(path_param.page))).execute(&conn)?;
    Ok(HttpResponse::NoContent())
}
