//! MongoDB implementation of UserRepository

use mongodb::{bson::doc, Collection, Database};
use sats_escrow_core::{
    error::{Error, Result},
    traits::repository::{BoxFuture, UserRepository},
    user::{User, UserId, UserRole},
};

/// MongoDB-backed user repository
pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    /// Create a new MongoDB user repository
    pub fn new(database: Database) -> Self {
        let collection = database.collection("users");
        Self { collection }
    }
}

impl UserRepository for MongoUserRepository {
    fn create(&self, user: &User) -> BoxFuture<'_, Result<()>> {
        let user = user.clone();
        Box::pin(async move {
            self.collection
                .insert_one(user, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            Ok(())
        })
    }

    fn update(&self, user: &User) -> BoxFuture<'_, Result<()>> {
        let user = user.clone();
        Box::pin(async move {
            let filter = doc! { "id": user.id.0.to_string() };
            self.collection
                .replace_one(filter, user, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            Ok(())
        })
    }

    fn find_by_id(&self, id: &UserId) -> BoxFuture<'_, Result<Option<User>>> {
        let id = id.clone();
        Box::pin(async move {
            let filter = doc! { "id": id.0.to_string() };
            self.collection
                .find_one(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_arbitrators(&self) -> BoxFuture<'_, Result<Vec<User>>> {
        Box::pin(async move {
            use futures::TryStreamExt;
            // Find users with Arbitrator role
            let filter = doc! {
                "roles": "Arbitrator"
            };
            let cursor = self
                .collection
                .find(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            cursor
                .try_collect()
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }
}

// Suppress unused warning for UserRole import (used in doc comments)
const _: () = {
    fn _uses_user_role(_: UserRole) {}
};
