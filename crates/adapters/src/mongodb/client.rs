//! MongoDB client wrapper for connection management

use mongodb::{Client, Database, options::ClientOptions};
use tracing::info;

/// MongoDB client wrapper with connection pooling
#[derive(Clone)]
pub struct MongoClient {
    client: Client,
    database: Database,
}

impl MongoClient {
    /// Create a new MongoDB client
    pub async fn new(uri: &str, database_name: &str) -> Result<Self, mongodb::error::Error> {
        let options = ClientOptions::parse(uri).await?;
        let client = Client::with_options(options)?;
        let database = client.database(database_name);
        
        // Test connection
        database.run_command(mongodb::bson::doc! { "ping": 1 }, None).await?;
        info!("Connected to MongoDB database: {}", database_name);
        
        Ok(Self { client, database })
    }

    /// Get the underlying database reference
    pub fn database(&self) -> &Database {
        &self.database
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
