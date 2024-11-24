use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod api;
mod database;
mod utils;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    let products_api = api::products::ProductApi::new(pool.clone());
    let categories_api = api::category::CategoryApi::new(pool.clone());

    let api_service = OpenApiService::new((categories_api,products_api), "КондиДария API", "1.0").server("http://localhost:8000/api");

    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    let app = Route::new().nest("/api", api_service).nest("/", ui).nest("/openapi.json", spec_json);
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;

    Ok(())
}