mod gcdatastore;
mod gcp;
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

    // dbg!(gcp::get_access_token("./credentials.json").await);
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
        let datastore_url = std::env::var("GCP_DATASTORE_URL")
            .unwrap_or("https://datastore.googleapis.com".to_owned());
        let project_id = std::env::var("GCP_PROJECT_ID").unwrap();
        let token_getter = if std::env::var("ENV").unwrap_or("dev".to_string()) == "dev" {
            gcdatastore::TokenGetter::Dummy
        } else {
            gcdatastore::TokenGetter::ACD
            // gcdatastore::TokenGetter::ServiceAccount(gcp::TokenGetter::from_credentials_json(&cred))
        };
        let gcds = gcdatastore::Client::new(datastore_url, project_id, token_getter);

        let schema = Schema::build(schema::Query, schema::Mutation, EmptySubscription)
            .data(gcds)
            .extension(async_graphql::extensions::Logger)
            .finish();

        App::new()
            .wrap(new_auth())
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
    .bind(std::env::var("ADDRESS").unwrap_or(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8080".to_owned())
    )))?
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

fn new_auth() -> actix_web_httpauth::middleware::HttpAuthentication<
    actix_web_httpauth::extractors::bearer::BearerAuth,
    Box<
        dyn Fn(
            actix_web::dev::ServiceRequest,
            actix_web_httpauth::extractors::bearer::BearerAuth,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                    Output = std::result::Result<
                        actix_web::dev::ServiceRequest,
                        (actix_web::Error, actix_web::dev::ServiceRequest),
                    >,
                >,
            >,
        >,
    >,
> {
    let token = std::env::var("ACCESS_TOKEN").ok();
    let process_fn: Box<
        dyn Fn(
            actix_web::dev::ServiceRequest,
            actix_web_httpauth::extractors::bearer::BearerAuth,
        ) -> std::pin::Pin<_>,
    > = Box::new(
        move |req: actix_web::dev::ServiceRequest,
              auth: actix_web_httpauth::extractors::bearer::BearerAuth| {
            let token = token.clone();
            Box::pin(async move {
                if token.map(|t| t == auth.token()) == Some(true) {
                    Ok(req)
                } else {
                    let config = req
                        .app_data::<actix_web_httpauth::extractors::bearer::Config>()
                        .cloned()
                        .unwrap_or_default()
                        .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

                    Err((
                        actix_web_httpauth::extractors::AuthenticationError::from(config).into(),
                        req,
                    ))
                }
            })
                as std::pin::Pin<
                    Box<
                        dyn std::future::Future<
                            Output = std::result::Result<
                                actix_web::dev::ServiceRequest,
                                (actix_web::Error, actix_web::dev::ServiceRequest),
                            >,
                        >,
                    >,
                >
        },
    );
    let auth = actix_web_httpauth::middleware::HttpAuthentication::bearer(process_fn);
    auth
}
