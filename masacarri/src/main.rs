use std::env;
use std::time::Duration;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::SessionMiddleware;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::middleware::{Condition, Logger};
use actix_web::{
    cookie::Key, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;
use error::{AppError, AppResult};
use schema::users;
use serde::Deserialize;
use serde_json::json;

#[macro_use]
extern crate diesel;

mod bgtask;
mod comment;
mod db;
mod error;
mod mail;
mod models;
mod page;
mod schema;
mod utils;
use crate::comment::*;
use crate::db::*;
use crate::page::*;

#[derive(Deserialize)]
struct LoginRequest {
    user: String,
    password: String,
}

async fn login(
    request: HttpRequest,
    db: web::Data<Pool>,
    login_data: web::Json<LoginRequest>,
) -> AppResult<impl Responder> {
    let conn = db.get()?;

    let LoginRequest { user, password } = login_data.0;

    const LOGIN_ERR_MSG: &str = "login failed";

    let user_password_hash = users::dsl::users
        .filter(users::dsl::username.eq(user.as_str()))
        .select((users::dsl::password_hash,))
        .first::<(String,)>(&conn);

    let user_password_hash = if let Ok(h) = user_password_hash {
        h.0
    } else {
        return Err(AppError::AuthErr(LOGIN_ERR_MSG.to_string()));
    };

    if let Ok(true) = bcrypt::verify(password, &user_password_hash) {
        Identity::login(&request.extensions(), user.into()).unwrap();
        Ok(HttpResponse::Ok().json(json! {
            {
                "message": "successfully logged in"
            }
        }))
    } else {
        return Err(AppError::AuthErr(LOGIN_ERR_MSG.to_string()));
    }
}

async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().json(json! {
        {
            "message": "successfully logged out"
        }
    })
}

diesel_migrations::embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    {
        let db_conn = establish_main_db();
        let res = embedded_migrations::run_with_output(&db_conn, &mut std::io::stdout());
        match res {
            Ok(_) => {
                println!("successfully setup database");
            }
            Err(e) => {
                eprintln!("failed to database migration");
                return std::io::Result::Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("failed to database migration: {}", e.to_string()),
                ));
            }
        }
    }

    println!("Masacarri Server Starting...");

    let pool = establish_main_db_pool();
    println!("Connected to database");

    let secret_key = Key::generate();
    let redis_store = establish_session_db().await;
    println!("Connected to session db");

    let host_self = env::var("HOST").expect("HOST must be set");
    let port_self = env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a valid unsigned 16-bit integer");

    let server = HttpServer::new(move || {
        let mode = env::var("MODE").unwrap_or("production".to_string());

        let cors = {
            let front_origin =
                env::var("FRONT_ORIGIN").unwrap_or("http://127.0.0.1:5173".to_string());

            Cors::default()
                .allowed_origin(front_origin.as_str())
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600)
        };

        let identity_middleware = IdentityMiddleware::builder()
            .login_deadline(Some(Duration::new(3600 * 3, 0)))
            .build();

        let session_middleware = SessionMiddleware::new(redis_store.clone(), secret_key.clone());

        let bgtask_manager = bgtask::make_bgtask_manager();

        App::new()
            .app_data(web::Data::new(bgtask_manager))
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(identity_middleware)
            .wrap(session_middleware)
            .wrap(Condition::new(mode == "development", cors))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::get().to(logout))
            .route("/api/pages", web::get().to(get_page_all))
            .route("/api/pages", web::post().to(add_page))
            .route("/api/pages/{page}", web::patch().to(modify_page))
            .route("/api/pages/{page}", web::delete().to(delete_page))
            .route("/api/pages/{page}/comments", web::get().to(get_comments))
            .route("/api/pages/{page}/comments", web::post().to(add_comment))
            .route(
                "/api/pages/{page}/comments/{comment}",
                web::get().to(get_comment),
            )
            .route(
                "/api/pages/{page}/comments/{comment}",
                web::patch().to(mark_comment),
            )
            .route(
                "/api/pages/{page}/comments_count",
                web::get().to(get_comment_count),
            )
            .service(
                actix_files::Files::new("/", "../masacarri-front/dist")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (http_req, _payload) = req.into_parts();
                        let response = NamedFile::open_async("../masacarri-front/dist/index.html")
                            .await?
                            .into_response(&http_req);
                        Ok(ServiceResponse::new(http_req, response))
                    })),
            )
    })
    .bind((host_self, port_self))?
    .run();

    println!("Server Started.");

    server.await
}
