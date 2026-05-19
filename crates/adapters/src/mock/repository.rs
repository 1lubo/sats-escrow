//! Mock in-memory repositories

use std::collections::HashMap;
use std::sync::RwLock;

use sats_escrow_core::{
    dispute::Dispute,
    escrow::Escrow,
    traits::repository::{BoxFuture, DisputeRepository, EscrowRepository, UserRepository},
    types::{DisputeId, EscrowId},
    user::{User, UserId, UserRole},
    Result,
};

/// Mock in-memory escrow repository
pub struct MockEscrowRepository {
    escrows: RwLock<HashMap<String, Escrow>>,
}

impl MockEscrowRepository {
    pub fn new() -> Self {
        Self {
            escrows: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MockEscrowRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl EscrowRepository for MockEscrowRepository {
    fn create(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>> {
        let escrow = escrow.clone();
        Box::pin(async move {
            let mut escrows = self.escrows.write().unwrap();
            escrows.insert(escrow.id.0.to_string(), escrow);
            Ok(())
        })
    }

    fn update(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>> {
        let escrow = escrow.clone();
        Box::pin(async move {
            let mut escrows = self.escrows.write().unwrap();
            escrows.insert(escrow.id.0.to_string(), escrow);
            Ok(())
        })
    }

    fn find_by_id(&self, id: &EscrowId) -> BoxFuture<'_, Result<Option<Escrow>>> {
        let id = id.clone();
        Box::pin(async move {
            let escrows = self.escrows.read().unwrap();
            Ok(escrows.get(&id.0.to_string()).cloned())
        })
    }

    fn find_by_buyer(&self, buyer: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let buyer = buyer.clone();
        Box::pin(async move {
            let escrows = self.escrows.read().unwrap();
            Ok(escrows
                .values()
                .filter(|e| e.buyer == buyer)
                .cloned()
                .collect())
        })
    }

    fn find_by_seller(&self, seller: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let seller = seller.clone();
        Box::pin(async move {
            let escrows = self.escrows.read().unwrap();
            Ok(escrows
                .values()
                .filter(|e| e.seller == seller)
                .cloned()
                .collect())
        })
    }

    fn find_by_user(&self, user: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        let user = user.clone();
        Box::pin(async move {
            let escrows = self.escrows.read().unwrap();
            Ok(escrows
                .values()
                .filter(|e| e.buyer == user || e.seller == user)
                .cloned()
                .collect())
        })
    }

    fn find_pending_auto_release(&self) -> BoxFuture<'_, Result<Vec<Escrow>>> {
        Box::pin(async move {
            let escrows = self.escrows.read().unwrap();
            Ok(escrows
                .values()
                .filter(|e| e.should_auto_release())
                .cloned()
                .collect())
        })
    }
}

/// Mock in-memory dispute repository
pub struct MockDisputeRepository {
    disputes: RwLock<HashMap<String, Dispute>>,
}

impl MockDisputeRepository {
    pub fn new() -> Self {
        Self {
            disputes: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MockDisputeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl DisputeRepository for MockDisputeRepository {
    fn create(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>> {
        let dispute = dispute.clone();
        Box::pin(async move {
            let mut disputes = self.disputes.write().unwrap();
            disputes.insert(dispute.id.0.to_string(), dispute);
            Ok(())
        })
    }

    fn update(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>> {
        let dispute = dispute.clone();
        Box::pin(async move {
            let mut disputes = self.disputes.write().unwrap();
            disputes.insert(dispute.id.0.to_string(), dispute);
            Ok(())
        })
    }

    fn find_by_id(&self, id: &DisputeId) -> BoxFuture<'_, Result<Option<Dispute>>> {
        let id = id.clone();
        Box::pin(async move {
            let disputes = self.disputes.read().unwrap();
            Ok(disputes.get(&id.0.to_string()).cloned())
        })
    }

    fn find_by_escrow(&self, escrow_id: &EscrowId) -> BoxFuture<'_, Result<Option<Dispute>>> {
        let escrow_id = escrow_id.clone();
        Box::pin(async move {
            let disputes = self.disputes.read().unwrap();
            Ok(disputes
                .values()
                .find(|d| d.escrow_id == escrow_id)
                .cloned())
        })
    }

    fn find_open_disputes(&self) -> BoxFuture<'_, Result<Vec<Dispute>>> {
        Box::pin(async move {
            let disputes = self.disputes.read().unwrap();
            Ok(disputes
                .values()
                .filter(|d| matches!(d.state, sats_escrow_core::dispute::DisputeState::Opened))
                .cloned()
                .collect())
        })
    }

    fn find_by_arbitrator(&self, arbitrator: &UserId) -> BoxFuture<'_, Result<Vec<Dispute>>> {
        let arbitrator = arbitrator.clone();
        Box::pin(async move {
            let disputes = self.disputes.read().unwrap();
            Ok(disputes
                .values()
                .filter(|d| {
                    if let sats_escrow_core::dispute::DisputeState::InReview { arbitrators } =
                        &d.state
                    {
                        arbitrators.contains(&arbitrator)
                    } else {
                        false
                    }
                })
                .cloned()
                .collect())
        })
    }
}

/// Mock in-memory user repository
pub struct MockUserRepository {
    users: RwLock<HashMap<String, User>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MockUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository for MockUserRepository {
    fn create(&self, user: &User) -> BoxFuture<'_, Result<()>> {
        let user = user.clone();
        Box::pin(async move {
            let mut users = self.users.write().unwrap();
            users.insert(user.id.0.to_string(), user);
            Ok(())
        })
    }

    fn update(&self, user: &User) -> BoxFuture<'_, Result<()>> {
        let user = user.clone();
        Box::pin(async move {
            let mut users = self.users.write().unwrap();
            users.insert(user.id.0.to_string(), user);
            Ok(())
        })
    }

    fn find_by_id(&self, id: &UserId) -> BoxFuture<'_, Result<Option<User>>> {
        let id = id.clone();
        Box::pin(async move {
            let users = self.users.read().unwrap();
            Ok(users.get(&id.0.to_string()).cloned())
        })
    }

    fn find_arbitrators(&self) -> BoxFuture<'_, Result<Vec<User>>> {
        Box::pin(async move {
            let users = self.users.read().unwrap();
            Ok(users
                .values()
                .filter(|u| matches!(u.role, UserRole::Arbitrator | UserRole::Both))
                .cloned()
                .collect())
        })
    }
}
