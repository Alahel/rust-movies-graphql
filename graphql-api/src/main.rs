use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{GraphQLBadRequest, GraphQLResponse};
use models::{IMovies, QueryRoot};
use std::convert::Infallible;
use tokio_postgres::{Client, NoTls};
use warp::{http::Response as HttpResponse, http::StatusCode as WarpStatusCode, Filter, Rejection};

#[tokio::main]
async fn main() {
    // let settings = Config::builder()
    //     .add_source(config::File::with_name("settings"))
    //     .build()
    //     .unwrap();

    // let port: u16 = settings
    //     .try_deserialize::<HashMap<String, String>>()
    //     .unwrap().get_key_value(k);

    let port: u16 = 8000;

    let connection_result = tokio_postgres::connect(
        "postgresql://backoffice_rw:GmJ169Hi6kWznsS@vip-ac-pg-dbza-staging.allocine.net:6432/dbz",
        NoTls,
    )
    .await;

    let client: Client;
    match connection_result {
        Ok((c, connection)) => {
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("postgres connection error: {}", e);
                }
            });
            client = c;
        }
        Err(e) => {
            eprintln!("postgres connection error: {}", e);
            std::process::exit(1);
        }
    }

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(IMovies::new(client))
        .finish();

    println!("GraphiQL IDE: http://localhost:{port}", port = port);

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<QueryRoot, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(GraphQLResponse::from(schema.execute(request).await))
        },
    );

    let graphiql = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(GraphiQLSource::build().endpoint("/").finish())
    });

    let routes = graphiql
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(GraphQLBadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    WarpStatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                WarpStatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
