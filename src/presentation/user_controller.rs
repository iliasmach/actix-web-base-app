use web_app_skeleton::base::controller::BaseController;
use actix_web::web::ServiceConfig;
use actix_web::{web, Responder};
use crate::Application;
use crate::application::messages::user::UserExist;

pub struct UserController {}

impl UserController {
    async fn sync_signal(data:web::Data<Application>) -> impl Responder {

        let a = data.user_service.send(UserExist { id: 5 }).await.unwrap();

        format!("{:?}", a.unwrap())
    }
}

impl BaseController for UserController {
    fn configure(cfg: &mut ServiceConfig) {
        cfg.route("/sync_signal", web::get().to(Self::sync_signal));
    }
}