//! MongoDB client wrapper for connection management

use mongodb::{options::ClientOptions, Client, Database};
use tracing::info;

/// MongoDB client wrapper with connection pooling
#[derive(Clone)]
pub struct MongoClient {
    client: Client,
}

impl MongoClient {
    /// Connect to MongoDB using a connection string
    pub async fn connect(uri: &str) -> Result<Self, mongodb::error::Error> {
        let options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        Ok(Self { client })
    }

    /// Create a new MongoDB client (legacy API)
    pub async fn new(uri: &str, database_name: &str) -> Result<Self, mongodb::error::Error> {
        let client = Self::connect(uri).await?;

        // Test connection
        let db = client.database(database_name);
        db.run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await?;
        info!("Connected to MongoDB database: {}", database_name);

        Ok(client)
    }

    /// Ping the database to verify connection
    pub async fn ping(&self) -> Result<(), mongodb::error::Error> {
        self.client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 }, None)
            .await?;
        Ok(())
    }

    /// Get a database reference by name
    pub fn database(&self, name: &str) -> Database {
        self.client.database(name)
    }

    /// Get the underlying client reference
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get escrow collection name
    pub fn escrows_collection(&self) -> &'static str {
        "escrows"
    }

    /// Get disputes collection name
    pub fn disputes_collection(&self) -> &'static str {
        "disputes"
    }

    /// Get users collection name
    pub fn users_collection(&self) -> &'static str {
        "users"
    }
}

#[cfg(test)]
mod tests {
    // Integration tests would require a running MongoDB instance
}
