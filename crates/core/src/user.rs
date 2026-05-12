//! User and reputation types

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Unique identifier for a user
/// Serializes as a string for MongoDB compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for UserId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Support both string and UUID binary deserialization
        struct UuidVisitor;

        impl<'de> serde::de::Visitor<'de> for UuidVisitor {
            type Value = UserId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UUID string or bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<UserId, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(value)
                    .map(UserId)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<UserId, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(value)
                    .map(UserId)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(UuidVisitor)
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
