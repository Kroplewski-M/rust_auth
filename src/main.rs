mod auth;
mod config;
mod db;
mod dtos;
mod errors;
mod handler;
mod models;
mod utils;
use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, http::header, middleware::Logger, web,
};
use config::Config;
use dotenv::dotenv;
use dtos::{
    FilterUserDto, LoginUserDto, RegisterUserDto, Response, UserData, UserListResponseDto,
    UserLoginResponseDto, UserResponseDto,
};
use sqlx::postgres::PgPoolOptions;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::db::DbClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DbClient,
}

#[derive(OpenApi)]
#[openapi(paths(health_checker),
    components(schemas(UserData, FilterUserDto, LoginUserDto, RegisterUserDto, UserResponseDto,UserLoginResponseDto,RegisterUserDto, UserListResponseDto)),
    tags((name = "Rust Auth API", description = "Auth in rust API")))]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    unsafe {
        openssl_probe::try_init_openssl_env_vars();
    }
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    env_logger::init();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations excetued successfully"),
        Err(e) => println!("Error running migrations: {}", e),
    }
    let db_client = DbClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };
    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(health_checker)
            .service(Redoc::with_url("/redoc", openapi.clone()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
            .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}

#[utoipa::path(get,path="/api/healthchecker",tag="health checker", responses((status = 200, description = "Authenticated user", body = Response)))]
#[get("/api/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "rust auth api";
    HttpResponse::Ok().json(serde_json::json!({"status":"success", "message": MESSAGE}))
}
