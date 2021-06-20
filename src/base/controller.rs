use actix_web::web;

pub trait BaseController {
    fn configure(cfg: &mut web::ServiceConfig);
}