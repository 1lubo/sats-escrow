//! Dispute entity and resolution types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    types::{DisputeId, EscrowId, Evidence, Party},
    user::UserId,
};

/// State of a dispute
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisputeState {
    /// Dispute opened, awaiting arbitrator assignment
    Opened,
    /// Arbitrators assigned, voting in progress
    InReview { arbitrators: Vec<UserId> },
    /// Voting complete, resolved
    Resolved { resolution: Resolution },
}

/// Resolution of a dispute
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub winner: Party,
    pub votes_for_buyer: u32,
    pub votes_for_seller: u32,
    pub resolved_at: DateTime<Utc>,
}

/// A vote by an arbitrator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub arbitrator: UserId,
    pub decision: Party,
    pub reasoning: Option<String>,
    pub voted_at: DateTime<Utc>,
}

/// The dispute entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub id: DisputeId,
    pub escrow_id: EscrowId,
    pub state: DisputeState,
    pub opened_by: UserId,
    pub buyer_evidence: Evidence,
    pub seller_evidence: Option<Evidence>,
    pub votes: Vec<Vote>,
    pub created_at: DateTime<Utc>,
    pub review_deadline: Option<DateTime<Utc>>,
}

impl Dispute {
    /// Create a new dispute
    pub fn new(escrow_id: EscrowId, opened_by: UserId, evidence: Evidence) -> Self {
        Self {
            id: DisputeId::new(),
            escrow_id,
            state: DisputeState::Opened,
            opened_by,
            buyer_evidence: evidence,
            seller_evidence: None,
            votes: Vec::new(),
            created_at: Utc::now(),
            review_deadline: None,
        }
    }

    /// Assign arbitrators and start review
    pub fn assign_arbitrators(&mut self, arbitrators: Vec<UserId>, review_hours: i64) {
        self.state = DisputeState::InReview {
            arbitrators: arbitrators.clone(),
        };
        self.review_deadline = Some(Utc::now() + chrono::Duration::hours(review_hours));
    }

    /// Submit seller's counter-evidence
    pub fn submit_seller_evidence(&mut self, evidence: Evidence) {
        self.seller_evidence = Some(evidence);
    }

    /// Record a vote
    pub fn record_vote(&mut self, vote: Vote) -> crate::Result<()> {
        if !matches!(self.state, DisputeState::InReview { .. }) {
            return Err(crate::Error::DisputeNotVotable);
        }

        if self.votes.iter().any(|v| v.arbitrator == vote.arbitrator) {
            return Err(crate::Error::AlreadyVoted);
        }

        self.votes.push(vote);
        self.check_resolution();

        Ok(())
    }

    /// Check if we have enough votes to resolve
    fn check_resolution(&mut self) {
        let required_votes = match &self.state {
            DisputeState::InReview { arbitrators } => (arbitrators.len() / 2) + 1,
            _ => return,
        };

        let buyer_votes = self.votes.iter().filter(|v| v.decision == Party::Buyer).count() as u32;
        let seller_votes = self.votes.iter().filter(|v| v.decision == Party::Seller).count() as u32;

        // Check for majority
        if buyer_votes >= required_votes as u32 {
            self.state = DisputeState::Resolved {
                resolution: Resolution {
                    winner: Party::Buyer,
                    votes_for_buyer: buyer_votes,
                    votes_for_seller: seller_votes,
                    resolved_at: Utc::now(),
                },
            };
        } else if seller_votes >= required_votes as u32 {
            self.state = DisputeState::Resolved {
                resolution: Resolution {
                    winner: Party::Seller,
                    votes_for_buyer: buyer_votes,
                    votes_for_seller: seller_votes,
                    resolved_at: Utc::now(),
                },
            };
        }
    }

    /// Get the resolution if dispute is resolved
    pub fn resolution(&self) -> Option<&Resolution> {
        match &self.state {
            DisputeState::Resolved { resolution } => Some(resolution),
            _ => None,
        }
    }
}
