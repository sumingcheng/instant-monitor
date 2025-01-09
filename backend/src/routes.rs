use actix_web::web;
mod system;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/system").configure(system::config));
}
