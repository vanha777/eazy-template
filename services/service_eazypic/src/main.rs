pub mod handler;
pub mod models;
pub mod routes;
pub mod utilities;

use axum::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use dotenv::dotenv;
use lib_sharedstate::ServerState;
use mongodb::{options::ClientOptions, Client};
use std::{env, net::SocketAddr, str::FromStr, sync::Arc};
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use module_aws::utilities::get_aws_client;
#[tokio::main]
async fn main() {
    // Load the .env file
    dotenv().ok();

    // Get the PORT variable from the environment, or default to "1010" if not set
    let port = u16::from_str(&env::var("PORT").unwrap_or_else(|_| "1010".to_string()))
        .expect("Invalid port number");

    //create mongo cilent
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let mongo_client = Client::with_options(client_options)
        .unwrap_or_else(|e| panic!("Failed to initialize mongodb client {:?}", e.to_string()));

    //create postgres client
    let database_url = "postgresql://postgres:mysecretpassword@localhost:5432/postgres";
    let manager = PostgresConnectionManager::new(database_url.parse().unwrap(), NoTls);
    let pool = Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create pool.");

    //create openAi client
    let openai_client = async_openai::Client::new();
    //creta HEYGEN client
    let heygen_client = env::var("HEYGEN_API_KEY").unwrap_or_default();
    // create aws Client
    let aws_client = get_aws_client().await.unwrap();
    //State that shared accors all routes
    let state = Arc::new(ServerState {
        mongo_client,
        sql_client: pool,
        openai_client,
        heygen_client,
        aws_client,
    });
    //main entry point for the server
    let service = routes::routes().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CompressionLayer::new().gzip(true))
            .layer(CorsLayer::permissive())
            // Add middleware that inserts the state into all incoming request's
            // extensions.
            .layer(Extension(state)),
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
