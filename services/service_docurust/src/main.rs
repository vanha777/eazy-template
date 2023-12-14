pub mod handler;
pub mod models;
pub mod routes;

use dotenv::dotenv;
use std::{env, net::SocketAddr, str::FromStr};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();
    // run script ...

    // Get the PORT variable from the environment, or default to "1010" if not set
    let port = u16::from_str(&env::var("PORT").unwrap_or_else(|_| "3070".to_string()))
        .expect("Invalid port number");

    //main entry point for the server
    let service = routes::routes().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().gzip(true))
            .layer(CorsLayer::permissive()),
    );
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!(
        r#"
    
        .------.
        | o  o |
        |  {}  | < - Me, shocked at how awesome this server is on {}!
        `------`
    
        "#,
        "{}", port
    );

    //bind the server to the port
    axum_server::bind(addr)
        .serve(service.into_make_service())
        .await
        .unwrap();
}
