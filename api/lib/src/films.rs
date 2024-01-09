use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

async fn get_all() -> HttpResponse {
    HttpResponse::Ok().body("get_all")
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().body("get")
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().body("post")
}

async fn put() -> HttpResponse {
    HttpResponse::Ok().body("put")
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("delete")
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
            .route("", web::get().to(get_all))
            .route("/{film_id}", web::get().to(get))
            .route("", web::post().to(post))
            .route("", web::put().to(put))
            .route("/{film_id}", web::delete().to(delete)),
    );
}
