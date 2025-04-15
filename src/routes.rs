use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/api/wallet")
            .route(web::post().to(crate::handlers::create_wallet))
    );
}