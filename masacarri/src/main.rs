use std::env;

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::SessionMiddleware;
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{cookie::Key, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, http};
use dotenv::dotenv;
use serde_json::json;

#[macro_use]
extern crate diesel;

mod error;
mod comment;
mod db;
mod models;
mod page;
mod schema;
mod utils;
use crate::comment::*;
use crate::db::*;
use crate::page::*;

async fn login(request: HttpRequest) -> impl Responder {
    Identity::login(&request.extensions(), "User1".into()).unwrap();
    HttpResponse::Ok().json(json! {
        {
            "message": "successfully logged in"
        }
    })
}

async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok().json(json! {
        {
            "message": "successfully logged out"
        }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Masacarri Server Starting...");

    let pool = establish_main_db();
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

        let cors = if mode == "development" {
            let front_origin = env::var("FRONT_ORIGIN").unwrap_or("http://127.0.0.1:5173".to_string());

            Cors::default()
                .allowed_origin(front_origin.as_str())
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::CONTENT_TYPE,
                ])
                .supports_credentials()
                .max_age(3600)
        } else {
            Cors::default()
        };

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(cors)
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::get().to(logout))
            .route("/api/pages", web::get().to(get_page_all))
            .route("/api/pages", web::post().to(add_page))
            .route("/api/pages/{page}", web::patch().to(modify_page))
            .route("/api/pages/{page}", web::delete().to(delete_page))
            .route("/api/pages/{page}/comments", web::get().to(get_comment))
            .route("/api/pages/{page}/comments", web::post().to(add_comment))
            .route("/api/pages/{page}/comments/{comment}", web::patch().to(mark_comment))
            .route("/api/pages/{page}/comments_count", web::get().to(get_comment_count))
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
