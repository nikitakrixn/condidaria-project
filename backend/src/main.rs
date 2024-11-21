use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod api;
mod database;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Настройка подключения к базе данных
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Создание API с репозиторием
    let products_api = api::products::ProductApi::new(pool.clone());

    let api_service = OpenApiService::new(products_api, "My API", "1.0").server("0.0.0.0:8080");

    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/api", api_service).nest("/", ui);
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await?;

    Ok(())
}