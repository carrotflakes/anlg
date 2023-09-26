use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

use anlg_api::{
    clients::{gcdatastore, gpt},
    middlewares, schema,
};

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

    HttpServer::new(move || {
        let datastore_url = std::env::var("GCP_DATASTORE_URL")
            .unwrap_or("https://datastore.googleapis.com".to_owned());
        let project_id = std::env::var("GCP_PROJECT_ID").unwrap();
        let token_getter = if env == "dev" {
            gcdatastore::TokenGetter::Dummy
        } else {
            gcdatastore::TokenGetter::ACD
            // gcdatastore::TokenGetter::ServiceAccount(gcp::TokenGetter::from_credentials_json(&cred))
        };
        let datastore = gcdatastore::Client::new(datastore_url, project_id, token_getter);

        let openai_api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let gpt = gpt::Gpt::new(openai_api_key);

        let schema = Schema::build(schema::Query, schema::Mutation, EmptySubscription)
            .data(datastore)
            .data(gpt)
            .extension(async_graphql::extensions::Logger)
            .finish();

        let token = std::env::var("ACCESS_TOKEN").ok();

        App::new()
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .wrap(middlewares::new_auth(token))
                    .to(GraphQL::new(schema)),
            )
            .service(
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
