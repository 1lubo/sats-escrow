//! MongoDB implementation of EscrowRepository

use mongodb::{bson::doc, Collection, Database};
use sats_escrow_core::{
    escrow::Escrow,
    error::{Error, Result},
    traits::repository::{BoxFuture, EscrowRepository},
    types::EscrowId,
    user::UserId,
};
use tracing::{debug, error};

/// MongoDB-backed escrow repository
pub struct MongoEscrowRepository {
    collection: Collection<Escrow>,
}

impl MongoEscrowRepository {
    /// Create a new MongoDB escrow repository
    pub fn new(database: Database) -> Self {
        let collection = database.collection("escrows");
        Self { collection }
    }
}

impl EscrowRepository for MongoEscrowRepository {
    fn create(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>> {
        let escrow = escrow.clone();
        Box::pin(async move {
            debug!("Creating escrow with id: {:?}", escrow.id);
            match self.collection.insert_one(escrow, None).await {
                Ok(result) => {
                    debug!("Escrow inserted with MongoDB _id: {:?}", result.inserted_id);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to insert escrow: {}", e);
                    Err(Error::Repository(e.to_string()))
                }
            }
        })
    }

    fn update(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>> {
        let escrow = escrow.clone();
        Box::pin(async move {
            let filter = doc! { "id": escrow.id.0.to_string() };
            self.collection
                .replace_one(filter, escrow, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            Ok(())
        })
    }

    fn find_by_id(&self, id: &EscrowId) -> BoxFuture<'_, Result<Option<Escrow>>> {
        let id = id.clone();
        Box::pin(async move {
            let filter = doc! { "id": id.0.to_string() };
            self.collection
                .find_one(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_by_buyer(&self, buyer: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let buyer = buyer.clone();
        Box::pin(async move {
            use futures::TryStreamExt;
            let filter = doc! { "buyer": buyer.0.to_string() };
            let cursor = self.collection
                .find(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            cursor
                .try_collect()
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_by_seller(&self, seller: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let seller = seller.clone();
        Box::pin(async move {
            use futures::TryStreamExt;
            let filter = doc! { "seller": seller.0.to_string() };
            let cursor = self.collection
                .find(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            cursor
                .try_collect()
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_by_user(&self, user: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let user = user.clone();
        let collection = self.collection.clone();
        Box::pin(async move {
            use futures::TryStreamExt;
            let filter = doc! {
                "$or": [
                    { "buyer": user.0.to_string() },
                    { "seller": user.0.to_string() }
                ]
            };
            debug!("Finding escrows for user {}", user.0);
            let cursor = collection
                .find(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            let results: Vec<Escrow> = cursor
                .try_collect()
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            debug!("Found {} escrows for user", results.len());
            Ok(results)
        })
    }

    fn find_pending_auto_release(&self) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        Box::pin(async move {
            use futures::TryStreamExt;
            // Find escrows in AwaitingDelivery state
            let filter = doc! {
                "state": { "$regex": "^AwaitingDelivery" }
            };
            let cursor = self.collection
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
