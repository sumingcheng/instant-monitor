use actix_web::{get, web, HttpResponse, Responder};
use crate::services::system::SystemService;

pub fn config(cfg: &mut web::ServiceConfig) {
    let system_service = web::Data::new(SystemService::new());
    cfg.app_data(system_service.clone())
        .service(get_info);
}

#[get("/info")]
async fn get_info(service: web::Data<SystemService>) -> impl Responder {
    HttpResponse::Ok().json(service.get_system_info())
}
