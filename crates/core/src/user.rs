//! User and reputation types

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a user
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub display_name: String,
    pub reputation: ReputationScore,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub role: UserRole,
}

/// User role in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    /// Regular user who can buy/sell
    Participant,
    /// Can participate in dispute resolution
    Arbitrator,
    /// Both participant and arbitrator
    Both,
}

/// Reputation score for a user
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ReputationScore {
    /// Number of successful transactions
    pub successful_transactions: u32,
    /// Number of disputes where user was at fault
    pub disputes_lost: u32,
    /// For arbitrators: accuracy of votes
    pub arbitration_accuracy: Option<f64>,
    /// Overall score (0.0 - 1.0)
    pub score: f64,
}

impl Default for ReputationScore {
    fn default() -> Self {
        Self {
            successful_transactions: 0,
            disputes_lost: 0,
            arbitration_accuracy: None,
            score: 0.5, // Neutral starting score
        }
    }
}

impl ReputationScore {
    /// Calculate updated score based on transaction outcome
    pub fn record_success(&mut self) {
        self.successful_transactions += 1;
        self.recalculate();
    }

    pub fn record_dispute_loss(&mut self) {
        self.disputes_lost += 1;
        self.recalculate();
    }

    fn recalculate(&mut self) {
        let total = self.successful_transactions + self.disputes_lost;
        if total > 0 {
            self.score = self.successful_transactions as f64 / total as f64;
        }
    }
}
