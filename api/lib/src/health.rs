use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

// ? Method 1
// #[get("/health")]
// async fn health() -> HttpResponse {
//     HttpResponse::Ok()
//         .append_header(("x-version", "gamma"))
//         .append_header(("x-thankyou", "everyone live"))
//         .finish()
// }

// ? Method 2: using macros
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("x-version", "gamma"))
        .append_header(("x-thankyou", "everyone live"))
        .finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    // ? Method 1
    // cfg.service(hello_world).service(version).service(health);

    // ? Method 2: using macros
    cfg.route("/health", web::get().to(health));
}
