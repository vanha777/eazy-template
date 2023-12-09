pub mod aws;
pub mod error;
pub mod heygen;
pub mod openAi;
pub mod photoshop;
use aws_sdk_s3::Client as S3Client;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

// Some shared state used throughout our application
#[derive(Clone, Debug)]
pub struct ServerState {
    pub mongo_client: mongodb::Client,
    pub sql_client: Pool<PostgresConnectionManager<NoTls>>,
    pub openai_client: async_openai::Client<async_openai::config::OpenAIConfig>,
    pub heygen_client: String,
    // The AWS SDK clients are designed to be thread-safe and efficient
    // No needed pool connections
    pub aws_client: S3Client,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RequestInput {
    pub data: Option<String>,
    pub id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ImageJobReply {
    pub status: Option<String>,
    pub file_name: Option<String>,
    pub link: Option<String>,
}
