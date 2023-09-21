mod gcdatastore;
mod schema;

use actix_cors::Cors;
use actix_web::{guard, http, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_actix_web::GraphQL;

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("GraphiQL IDE: http://localhost:8000/graphql");

    // dbg!(
    //     gcdatastore::Client::new()
    //         .run_query::<Value>(&json!({
    //             "query": {
    //                 "limit": 50,
    //                 "kind": [{
    //                     "name": "note"
    //                 }]
    //             }
    //         }))
    //         .await
    // );

    HttpServer::new(move || {
        let gcds = gcdatastore::Client::new();

        let schema = Schema::build(schema::Query, schema::Mutation, EmptySubscription)
            .data(gcds)
            .extension(async_graphql::extensions::Logger)
            .finish();

        App::new()
            .wrap(new_cors())
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(GraphQL::new(schema)),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_graphiql),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

fn new_cors() -> Cors {
    let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or("*".to_owned());

    let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600);
    if cors_origin == "*" {
        cors.allow_any_origin().supports_credentials()
    } else {
        cors.allowed_origin(&cors_origin)
    }
}
