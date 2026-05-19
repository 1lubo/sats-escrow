//! Server configuration
//!
//! Configuration is loaded from environment variables:
//! - `PORT` - HTTP server port (default: 3000)
//! - `MONGODB_URI` - MongoDB connection string (optional, uses mock if not set)
//! - `DATABASE_NAME` - MongoDB database name (default: sats_escrow)
//! - `ALLOWED_ORIGINS` - Comma-separated list of allowed CORS origins (optional, allows any if not set)
//! - `RUST_LOG` - Log level (default: info)

use std::env;

/// Server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// HTTP server port
    pub port: u16,
    /// MongoDB connection URI (None = use mock adapters)
    pub mongodb_uri: Option<String>,
    /// MongoDB database name
    pub database_name: String,
    /// Allowed CORS origins (None = allow any origin)
    pub allowed_origins: Option<Vec<String>>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            mongodb_uri: env::var("MONGODB_URI").ok(),
            database_name: env::var("DATABASE_NAME").unwrap_or_else(|_| "sats_escrow".to_string()),
            allowed_origins: env::var("ALLOWED_ORIGINS").ok().map(|origins| {
                origins
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            }),
        }
    }

    /// Check if MongoDB is configured
    pub fn use_mongodb(&self) -> bool {
        self.mongodb_uri.is_some()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env()
    }
}
