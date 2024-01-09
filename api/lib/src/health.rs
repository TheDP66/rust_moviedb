use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse,
};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

// #[get("/version")]
// async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
//     tracing::info!("Getting version");
//     let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
//         .fetch_one(db.get_ref())
//         .await;

//     match result {
//         Ok(version) => version,
//         Err(e) => format!("Error: {:?}", e),
//     }
// }

async fn version() -> HttpResponse {
    tracing::info!("Getting version");
    HttpResponse::Ok()
        .append_header(("x-version", API_VERSION))
        .append_header(("x-thankyou", "everyone live"))
        .body(API_VERSION)
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
        .append_header(("x-version", API_VERSION))
        .append_header(("x-thankyou", "everyone live"))
        .finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    // ? Method 1
    // cfg.service(hello_world).service(version).service(health);

    // ? Method 2: using macros
    cfg.route("/health", web::get().to(health))
        .route("/version", web::get().to(version));
}

pub const API_VERSION: &str = "v0.0.1";

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    // ? Unit test example
    #[actix_rt::test]
    async fn health_check_works() {
        let res = version().await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res.headers().get("x-version").and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
