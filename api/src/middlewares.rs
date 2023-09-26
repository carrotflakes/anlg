use actix_cors::Cors;
use actix_web::http;

pub fn new_cors() -> Cors {
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

/// If `token` is `None`, then no authentication is required.
pub fn new_auth(
    token: Option<String>,
) -> actix_web_httpauth::middleware::HttpAuthentication<
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
                if token.is_none() || token.map(|t| t == auth.token()) == Some(true) {
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
