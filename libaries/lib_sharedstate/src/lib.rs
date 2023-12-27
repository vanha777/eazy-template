use aws_sdk_s3::Client as S3Client;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

// Some shared state used throughout our application
#[derive(Clone, Debug)]
pub struct ServerState {
    pub mongo_client: mongodb::Client,
    pub sql_client: mysql_async::Pool,
    pub openai_client: async_openai::Client<async_openai::config::OpenAIConfig>,
    pub heygen_client: String,
    // The AWS SDK clients are designed to be thread-safe and efficient
    // No needed pool connections
    pub aws_client: S3Client,
}
