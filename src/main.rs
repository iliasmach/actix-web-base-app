extern crate diesel;
extern crate diesel_migrations;

use actix_web::{HttpServer, App, web};
use crate::presentation::user_controller::UserController;
use web_app_skeleton::base::controller::BaseController;

use crate::infra::repository::UserRepositoryDB;
use crate::infra::db;
use crate::application::app::Application;

pub mod base;
pub mod domain;
pub mod infra;
pub mod presentation;
pub mod application;


#[allow(unused_must_use)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv();
    //db::init();

    HttpServer::new(move || {
        let app = Application::new();
        App::new().data(app).service(web::scope("/api")
            .configure(UserController::configure))
    })
        .bind("127.0.0.1:8082")?
        .run()
        .await
}