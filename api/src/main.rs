use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

use anlg_api::{clients::gpt, middlewares, schema};
use firestore::{FirestoreDb, FirestoreDbOptions};

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let env = std::env::var("ENV").unwrap_or("dev".to_string());
    log::info!("ENV: {}", env);
    log::info!("GraphiQL IDE: http://localhost:8000/graphql");

    let project_id = std::env::var("GCP_PROJECT_ID").expect("GCP_PROJECT_ID is not set");
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    let token = std::env::var("ACCESS_TOKEN").ok();

    let mut db_opt = FirestoreDbOptions::new(project_id.to_string());
    db_opt = db_opt.with_database_id("prd-anlg".to_string());
    let db = FirestoreDb::with_options(db_opt).await.unwrap();

    HttpServer::new(move || {
        let repository = anlg_api::repository::Repository::new(db.clone());

        let gpt = gpt::new_gpt(openai_api_key.clone());

        let schema = Schema::build(schema::Query, schema::Mutation, EmptySubscription)
            .data(repository)
            .data(gpt)
            .extension(async_graphql::extensions::Logger)
            .finish();

        let mut app = App::new();
        app = if let Some(token) = token.clone() {
            app.service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .wrap(middlewares::new_auth(token))
                    .to(GraphQL::new(schema)),
            )
        } else {
            app.service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
        };
        app.service(
            web::resource("/graphiql")
                .guard(guard::Get())
                .to(index_graphiql),
        )
        .wrap(middlewares::new_cors())
    })
    .bind(std::env::var("ADDRESS").unwrap_or(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8080".to_owned())
    )))?
    .run()
    .await
}
