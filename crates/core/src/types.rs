//! Common types used throughout the domain

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

/// Unique identifier for an escrow
/// Serializes as a string for MongoDB compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EscrowId(pub Uuid);

impl EscrowId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EscrowId {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for EscrowId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for EscrowId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UuidVisitor;

        impl<'de> serde::de::Visitor<'de> for UuidVisitor {
            type Value = EscrowId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UUID string or bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<EscrowId, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(value)
                    .map(EscrowId)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<EscrowId, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(value)
                    .map(EscrowId)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(UuidVisitor)
    }
}

/// Unique identifier for a dispute
/// Serializes as a string for MongoDB compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DisputeId(pub Uuid);

impl DisputeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DisputeId {
    fn default() -> Self {
        Self::new()
    }
}

impl Serialize for DisputeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for DisputeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UuidVisitor;

        impl<'de> serde::de::Visitor<'de> for UuidVisitor {
            type Value = DisputeId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UUID string or bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<DisputeId, E>
            where
                E: serde::de::Error,
            {
                Uuid::parse_str(value)
                    .map(DisputeId)
                    .map_err(serde::de::Error::custom)
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<DisputeId, E>
            where
                E: serde::de::Error,
            {
                Uuid::from_slice(value)
                    .map(DisputeId)
                    .map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(UuidVisitor)
    }
}

/// Amount in satoshis (1 BTC = 100,000,000 satoshis)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Satoshis(pub u64);

impl Satoshis {
    pub fn from_btc(btc: f64) -> Self {
        Self((btc * 100_000_000.0) as u64)
    }

    pub fn to_btc(self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

/// Bitcoin address for deposits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepositAddress(pub String);

/// Bitcoin transaction ID
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxId(pub String);

/// Currency pair for exchange rate lookups
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrencyPair {
    pub base: String,
    pub quote: String,
}

impl CurrencyPair {
    pub fn btc_usd() -> Self {
        Self {
            base: "BTC".to_string(),
            quote: "USD".to_string(),
        }
    }
}

/// Exchange rate between currencies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExchangeRate(pub f64);

/// Invoice for payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub escrow_id: EscrowId,
    pub amount: Satoshis,
    pub deposit_address: DepositAddress,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Evidence submitted for a dispute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub description: String,
    pub attachments: Vec<String>, // URLs or references to attachments
}

/// Who initiated the escrow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Party {
    Buyer,
    Seller,
}
