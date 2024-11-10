use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{param::Query, payload::PlainText, OpenApi, OpenApiService};

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

    let api_service = OpenApiService::new(Api, "My API", "1.0").server("0.0.0.0:8080");

    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/api", api_service).nest("/", ui);
    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}