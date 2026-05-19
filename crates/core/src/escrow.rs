//! Escrow entity and state machine

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    types::{DepositAddress, DisputeId, EscrowId, Evidence, Party, Satoshis, TxId},
    user::UserId,
};

/// Escrow terms defining timeouts and conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowTerms {
    /// Duration after which funds auto-release to seller if no dispute
    pub auto_release_after: Duration,
    /// Window during which buyer can open a dispute after delivery
    pub dispute_window: Duration,
}

impl Default for EscrowTerms {
    fn default() -> Self {
        Self {
            auto_release_after: Duration::days(14),
            dispute_window: Duration::days(7),
        }
    }
}

/// State of an escrow
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscrowState {
    /// Escrow created, awaiting funding
    Created,
    /// Funds received, awaiting seller delivery
    Funded,
    /// Seller marked as delivered, awaiting buyer confirmation
    AwaitingDelivery { delivered_at: DateTime<Utc> },
    /// Dispute opened
    Disputed { dispute_id: DisputeId },
    /// Cancelled before funding
    Cancelled { reason: CancelReason },
    /// Funds released to seller
    ReleasedToSeller { tx_id: TxId },
    /// Funds released to buyer (refund)
    ReleasedToBuyer { tx_id: TxId },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancelReason {
    BuyerCancelled,
    SellerCancelled,
    MutualAgreement,
    Expired,
}

/// Event in escrow lifecycle for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: EscrowEventType,
    pub actor: Option<UserId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscrowEventType {
    Created,
    Funded { tx_id: TxId },
    DeliveryMarked,
    Confirmed,
    DisputeOpened { dispute_id: DisputeId },
    Cancelled { reason: CancelReason },
    Released { to: Party, tx_id: TxId },
}

/// The main escrow entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Escrow {
    pub id: EscrowId,
    pub state: EscrowState,
    pub initiator: Party,
    pub buyer: UserId,
    pub seller: UserId,
    pub amount: Satoshis,
    pub description: String,
    pub terms: EscrowTerms,
    pub deposit_address: Option<DepositAddress>,
    pub created_at: DateTime<Utc>,
    pub funded_at: Option<DateTime<Utc>>,
    pub events: Vec<EscrowEvent>,
}

impl Escrow {
    /// Create a new escrow
    pub fn new(
        initiator: Party,
        buyer: UserId,
        seller: UserId,
        amount: Satoshis,
        description: String,
        terms: EscrowTerms,
    ) -> Self {
        let id = EscrowId::new();
        let now = Utc::now();

        Self {
            id,
            state: EscrowState::Created,
            initiator,
            buyer,
            seller,
            amount,
            description,
            terms,
            deposit_address: None,
            created_at: now,
            funded_at: None,
            events: vec![EscrowEvent {
                timestamp: now,
                event_type: EscrowEventType::Created,
                actor: None,
            }],
        }
    }

    /// Set the deposit address (called after custodian creates it)
    pub fn set_deposit_address(&mut self, address: DepositAddress) {
        self.deposit_address = Some(address);
    }

    /// Mark escrow as funded
    pub fn mark_funded(&mut self, tx_id: TxId) -> Result<()> {
        if self.state != EscrowState::Created {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "Funded",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::Funded;
        self.funded_at = Some(now);
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::Funded { tx_id },
            actor: None,
        });

        Ok(())
    }

    /// Seller marks delivery
    pub fn mark_delivered(&mut self, actor: UserId) -> Result<()> {
        if self.state != EscrowState::Funded {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "AwaitingDelivery",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::AwaitingDelivery { delivered_at: now };
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::DeliveryMarked,
            actor: Some(actor),
        });

        Ok(())
    }

    /// Buyer confirms receipt, releasing funds to seller
    pub fn confirm(&mut self, actor: UserId, tx_id: TxId) -> Result<()> {
        if !matches!(self.state, EscrowState::AwaitingDelivery { .. }) {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "ReleasedToSeller",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::ReleasedToSeller {
            tx_id: tx_id.clone(),
        };
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::Released {
                to: Party::Seller,
                tx_id,
            },
            actor: Some(actor),
        });

        Ok(())
    }

    /// Open a dispute
    pub fn open_dispute(&mut self, actor: UserId, dispute_id: DisputeId) -> Result<()> {
        if !matches!(self.state, EscrowState::AwaitingDelivery { .. }) {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "Disputed",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::Disputed {
            dispute_id: dispute_id.clone(),
        };
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::DisputeOpened { dispute_id },
            actor: Some(actor),
        });

        Ok(())
    }

    /// Cancel escrow (only allowed before funding)
    pub fn cancel(&mut self, reason: CancelReason, actor: UserId) -> Result<()> {
        if self.state != EscrowState::Created {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "Cancelled",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::Cancelled {
            reason: reason.clone(),
        };
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::Cancelled { reason },
            actor: Some(actor),
        });

        Ok(())
    }

    /// Release to buyer (refund after dispute)
    pub fn release_to_buyer(&mut self, tx_id: TxId) -> Result<()> {
        if !matches!(self.state, EscrowState::Disputed { .. }) {
            return Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "ReleasedToBuyer",
            });
        }

        let now = Utc::now();
        self.state = EscrowState::ReleasedToBuyer {
            tx_id: tx_id.clone(),
        };
        self.events.push(EscrowEvent {
            timestamp: now,
            event_type: EscrowEventType::Released {
                to: Party::Buyer,
                tx_id,
            },
            actor: None,
        });

        Ok(())
    }

    /// Release to seller (after dispute or auto-release)
    pub fn release_to_seller(&mut self, tx_id: TxId) -> Result<()> {
        match &self.state {
            EscrowState::Disputed { .. } | EscrowState::AwaitingDelivery { .. } => {
                let now = Utc::now();
                self.state = EscrowState::ReleasedToSeller {
                    tx_id: tx_id.clone(),
                };
                self.events.push(EscrowEvent {
                    timestamp: now,
                    event_type: EscrowEventType::Released {
                        to: Party::Seller,
                        tx_id,
                    },
                    actor: None,
                });
                Ok(())
            }
            _ => Err(Error::InvalidStateTransition {
                from: self.state.clone(),
                to: "ReleasedToSeller",
            }),
        }
    }

    /// Check if auto-release timeout has passed
    pub fn should_auto_release(&self) -> bool {
        if let EscrowState::AwaitingDelivery { delivered_at } = &self.state {
            let deadline = *delivered_at + self.terms.auto_release_after;
            Utc::now() > deadline
        } else {
            false
        }
    }

    /// Check if dispute window is still open
    pub fn can_dispute(&self) -> bool {
        if let EscrowState::AwaitingDelivery { delivered_at } = &self.state {
            let deadline = *delivered_at + self.terms.dispute_window;
            Utc::now() <= deadline
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_buyer() -> UserId {
        UserId::new()
    }

    fn test_seller() -> UserId {
        UserId::new()
    }

    fn create_test_escrow() -> Escrow {
        Escrow::new(
            Party::Buyer,
            test_buyer(),
            test_seller(),
            Satoshis(100_000),
            "Test escrow".to_string(),
            EscrowTerms::default(),
        )
    }

    #[test]
    fn new_escrow_is_in_created_state() {
        let escrow = create_test_escrow();
        assert!(matches!(escrow.state, EscrowState::Created));
    }

    #[test]
    fn escrow_can_be_funded() {
        let mut escrow = create_test_escrow();
        let result = escrow.mark_funded(TxId("tx_123".to_string()));

        assert!(result.is_ok());
        assert!(matches!(escrow.state, EscrowState::Funded));
        assert!(escrow.funded_at.is_some());
    }

    #[test]
    fn funded_escrow_can_be_marked_delivered() {
        let mut escrow = create_test_escrow();
        escrow.mark_funded(TxId("tx_123".to_string())).unwrap();

        let seller = escrow.seller.clone();
        let result = escrow.mark_delivered(seller);

        assert!(result.is_ok());
        assert!(matches!(escrow.state, EscrowState::AwaitingDelivery { .. }));
    }

    #[test]
    fn created_escrow_cannot_be_marked_delivered() {
        let mut escrow = create_test_escrow();
        let seller = escrow.seller.clone();
        let result = escrow.mark_delivered(seller);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::Error::InvalidStateTransition { .. }
        ));
    }

    #[test]
    fn awaiting_delivery_escrow_can_be_confirmed() {
        let mut escrow = create_test_escrow();
        escrow.mark_funded(TxId("tx_123".to_string())).unwrap();

        let seller = escrow.seller.clone();
        escrow.mark_delivered(seller).unwrap();

        let buyer = escrow.buyer.clone();
        let result = escrow.confirm(buyer, TxId("tx_456".to_string()));

        assert!(result.is_ok());
        assert!(matches!(escrow.state, EscrowState::ReleasedToSeller { .. }));
    }

    #[test]
    fn created_escrow_can_be_cancelled() {
        let mut escrow = create_test_escrow();
        let buyer = escrow.buyer.clone();
        let result = escrow.cancel(CancelReason::BuyerCancelled, buyer);

        assert!(result.is_ok());
        assert!(matches!(escrow.state, EscrowState::Cancelled { .. }));
    }

    #[test]
    fn funded_escrow_cannot_be_cancelled() {
        let mut escrow = create_test_escrow();
        escrow.mark_funded(TxId("tx_123".to_string())).unwrap();

        let buyer = escrow.buyer.clone();
        let result = escrow.cancel(CancelReason::BuyerCancelled, buyer);

        assert!(result.is_err());
    }

    #[test]
    fn awaiting_delivery_can_be_disputed() {
        let mut escrow = create_test_escrow();
        escrow.mark_funded(TxId("tx_123".to_string())).unwrap();

        let seller = escrow.seller.clone();
        escrow.mark_delivered(seller).unwrap();

        let buyer = escrow.buyer.clone();
        let dispute_id = DisputeId::new();
        let result = escrow.open_dispute(buyer, dispute_id);

        assert!(result.is_ok());
        assert!(matches!(escrow.state, EscrowState::Disputed { .. }));
    }

    #[test]
    fn events_are_recorded() {
        let mut escrow = create_test_escrow();
        assert_eq!(escrow.events.len(), 1); // Created event

        escrow.mark_funded(TxId("tx_123".to_string())).unwrap();
        assert_eq!(escrow.events.len(), 2); // + Funded event

        let seller = escrow.seller.clone();
        escrow.mark_delivered(seller).unwrap();
        assert_eq!(escrow.events.len(), 3); // + Delivered event
    }
}
