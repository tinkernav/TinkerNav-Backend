mod login;
mod utils;
mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use login::{login, logout, register};

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
        .service(web::resource("/user").route(web::post().to(register)))
}
