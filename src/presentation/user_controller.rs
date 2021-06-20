use web_app_skeleton::base::controller::BaseController;
use actix_web::web::ServiceConfig;
use actix_web::{web, Responder};


use crate::infra::repository::UserRepositoryDB;
use crate::application::user_service::UserServiceImpl;

use std::ops::Deref;
use crate::Application;

pub struct UserController {}

impl UserController {
    async fn sync_signal(data:web::Data<Application>) -> impl Responder {
        let service = data.user_service.has_user(5);


        "OK"
    }
}

impl BaseController for UserController {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.route("/sync_signal", web::get().to(Self::sync_signal));
    }
}