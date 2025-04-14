use actix_web::web;
use crate::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/wallet")
            .route(web::post().to(handlers::create_wallet))
    );
}