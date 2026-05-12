//! MongoDB implementation of DisputeRepository

use mongodb::{bson::doc, Collection};
use sats_escrow_core::{
    dispute::Dispute,
    error::{Error, Result},
    traits::repository::{BoxFuture, DisputeRepository},
    types::{DisputeId, EscrowId},
    user::UserId,
};

use super::MongoClient;

/// MongoDB-backed dispute repository
pub struct MongoDisputeRepository {
    collection: Collection<Dispute>,
}

impl MongoDisputeRepository {
    /// Create a new MongoDB dispute repository
    pub fn new(client: &MongoClient) -> Self {
        let collection = client.database().collection(client.disputes_collection());
        Self { collection }
    }
}

impl DisputeRepository for MongoDisputeRepository {
    fn create(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>> {
        let dispute = dispute.clone();
        Box::pin(async move {
            self.collection
                .insert_one(dispute, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            Ok(())
        })
    }

    fn update(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>> {
        let dispute = dispute.clone();
        Box::pin(async move {
            let filter = doc! { "id": dispute.id.0.to_string() };
            self.collection
                .replace_one(filter, dispute, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))?;
            Ok(())
        })
    }

    fn find_by_id(&self, id: &DisputeId) -> BoxFuture<'_, Result<Option<Dispute>>> {
        let id = id.clone();
        Box::pin(async move {
            let filter = doc! { "id": id.0.to_string() };
            self.collection
                .find_one(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_by_escrow(&self, escrow_id: &EscrowId) -> BoxFuture<'_, Result<Option<Dispute>>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let filter = doc! { "escrow_id": escrow_id.0.to_string() };
            self.collection
                .find_one(filter, None)
                .await
                .map_err(|e| Error::Repository(e.to_string()))
        })
    }

    fn find_open_disputes(&self) -> BoxFuture<'_, Result<Vec<Dispute>>> {
        Box::pin(async move {
            use futures::TryStreamExt;
            // Find disputes in Opened or InReview state
            let filter = doc! {
                "$or": [
                    { "state": "Opened" },
                    { "state": { "$regex": "^InReview" } }
                ]
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

    fn find_by_arbitrator(&self, arbitrator: &UserId) -> BoxFuture<'_, Result<Vec<Dispute>>> {
        let arbitrator = arbitrator.clone();
        Box::pin(async move {
            use futures::TryStreamExt;
            // Search for arbitrator in the state.arbitrators array
            let filter = doc! {
                "state.InReview.arbitrators": arbitrator.0.to_string()
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
