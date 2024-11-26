use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod models;
mod api;
mod database;
mod utils;

const IS_DEVELOPMENT:bool = cfg!(debug_assertions);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let host_address = IS_DEVELOPMENT.then_some("localhost").unwrap_or("0.0.0.0");

    let listener_address = format!("{host_address}:8000");

    let listener = TcpListener::bind(listener_address);

    let cors = Cors::new().allow_origin("http://localhost:3000").allow_methods(["GET", "POST", "PUT", "DELETE", "OPTIONS"].iter().cloned());

    let products_api = api::products::ProductApi::new(pool.clone());
    let categories_api = api::category::CategoryApi::new(pool.clone());

    let api_service = OpenApiService::new((categories_api,products_api), "КондиДария API", "1.0").server("http://localhost:8000/api");

    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    let app = Route::new().nest("/api", api_service).nest("/", ui).nest("/openapi.json", spec_json).with(cors);
    
    Server::new(listener)
        .run(app)
        .await?;

    Ok(())
}