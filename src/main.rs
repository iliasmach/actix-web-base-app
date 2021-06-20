
extern crate diesel;

extern crate diesel_migrations;



use actix_web::{HttpServer, App, web};
use crate::presentation::user_controller::UserController;
use web_app_skeleton::base::controller::BaseController;
use crate::application::user_service::{UserServiceImpl};
use crate::infra::repository::UserRepositoryDB;
use crate::infra::db;


pub mod base;
pub mod domain;
pub mod infra;
pub mod presentation;
pub mod application;

pub struct Application {
    user_service: UserServiceImpl<UserRepositoryDB>,
}
#[allow(unused_must_use)]
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv();
    db::init();

    HttpServer::new(move || {
        let app = Application { user_service: UserServiceImpl::new(UserRepositoryDB::new()) };

        App::new().data(app).service(web::scope("/api")
            .configure(UserController::configure))
    })
        .bind("127.0.0.1:8082")?
        .run()
        .await
}