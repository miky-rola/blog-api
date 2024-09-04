use actix_web::{web, App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod schema;
mod orm;
mod routes;
mod api_doc;
mod db;

use api_doc::ApiDoc;
use db::DbPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let pool = db::create_db_pool();

    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone())
            )
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}