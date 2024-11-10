use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};
use sqlx::postgres::PgPoolOptions;

mod models;
mod api;
mod database;
struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello {}!", name)),
            None => PlainText("hello from there!".to_string()),
        }        
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    // Настройка подключения к базе данных
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Создание репозитория с пулом соединений
    let product_repository = database::product_repository::ProductRepository::new(pool.clone());

    // Создание API с репозиторием
    let products_api = api::products::ProductsApi::new(product_repository);

    let api_service = OpenApiService::new(products_api, "My API", "1.0").server("0.0.0.0:8080");

    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/api", api_service).nest("/", ui);
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}