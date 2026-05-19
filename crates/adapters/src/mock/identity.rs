//! Mock identity provider

use std::collections::HashMap;
use std::sync::RwLock;

use chrono::Utc;

use sats_escrow_core::{
    traits::identity::{BoxFuture, Credentials, IdentityProvider, Session},
    user::{ReputationScore, User, UserId, UserRole},
    Error, Result,
};

/// Mock identity provider with in-memory user storage
pub struct MockIdentityProvider {
    users: RwLock<HashMap<String, User>>,
    sessions: RwLock<HashMap<String, UserId>>,
    /// Username -> password (for mock auth)
    credentials: RwLock<HashMap<String, String>>,
}

impl MockIdentityProvider {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            sessions: RwLock::new(HashMap::new()),
            credentials: RwLock::new(HashMap::new()),
        }
    }

    /// Create a test user (for testing/demo purposes)
    pub fn create_test_user(&self, display_name: &str, role: UserRole) -> User {
        let user = User {
            id: UserId::new(),
            display_name: display_name.to_string(),
            reputation: ReputationScore::default(),
            created_at: Utc::now(),
            role,
        };

        let mut users = self.users.write().unwrap();
        users.insert(user.id.0.to_string(), user.clone());

        user
    }
}

impl Default for MockIdentityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityProvider for MockIdentityProvider {
    fn authenticate(&self, credentials: Credentials) -> BoxFuture<'_, Result<Session>> {
        Box::pin(async move {
            match credentials {
                Credentials::UsernamePassword { username, password } => {
                    let creds = self.credentials.read().unwrap();
                    if let Some(stored_password) = creds.get(&username) {
                        if stored_password == &password {
                            // Find user by username (simplified: username == display_name)
                            let users = self.users.read().unwrap();
                            if let Some(user) = users.values().find(|u| u.display_name == username)
                            {
                                let user_id = user.id.clone();
                                drop(users); // Release read lock before write

                                let token = format!("token_{}", uuid::Uuid::new_v4());
                                let mut sessions = self.sessions.write().unwrap();
                                sessions.insert(token.clone(), user_id.clone());

                                return Ok(Session {
                                    user_id,
                                    token,
                                    expires_at: Utc::now() + chrono::Duration::hours(24),
                                });
                            }
                        }
                    }
                    Err(Error::Unauthorized("Invalid credentials".to_string()))
                }
                Credentials::ApiKey(_key) => {
                    // For mock, accept any API key and create a session
                    let token = format!("token_{}", uuid::Uuid::new_v4());
                    let user_id = UserId::new();

                    let mut sessions = self.sessions.write().unwrap();
                    sessions.insert(token.clone(), user_id.clone());

                    Ok(Session {
                        user_id,
                        token,
                        expires_at: Utc::now() + chrono::Duration::hours(24),
                    })
                }
                Credentials::Nostr { pubkey, .. } => {
                    // For mock, accept any Nostr signature
                    let token = format!("nostr_{}", &pubkey[..8.min(pubkey.len())]);
                    let user_id = UserId::new();

                    let mut sessions = self.sessions.write().unwrap();
                    sessions.insert(token.clone(), user_id.clone());

                    Ok(Session {
                        user_id,
                        token,
                        expires_at: Utc::now() + chrono::Duration::hours(24),
                    })
                }
            }
        })
    }

    fn validate_session(&self, token: &str) -> BoxFuture<'_, Result<UserId>> {
        let token = token.to_string();
        Box::pin(async move {
            let sessions = self.sessions.read().unwrap();
            sessions
                .get(&token)
                .cloned()
                .ok_or_else(|| Error::Unauthorized("Invalid session".to_string()))
        })
    }

    fn get_user(&self, user_id: &UserId) -> BoxFuture<'_, Result<Option<User>>> {
        let user_id = user_id.clone();
        Box::pin(async move {
            let users = self.users.read().unwrap();
            Ok(users.get(&user_id.0.to_string()).cloned())
        })
    }

    fn get_reputation(&self, user_id: &UserId) -> BoxFuture<'_, Result<ReputationScore>> {
        let user_id = user_id.clone();
        Box::pin(async move {
            let users = self.users.read().unwrap();
            Ok(users
                .get(&user_id.0.to_string())
                .map(|u| u.reputation)
                .unwrap_or_default())
        })
    }

    fn update_reputation(
        &self,
        user_id: &UserId,
        score: ReputationScore,
    ) -> BoxFuture<'_, Result<()>> {
        let user_id = user_id.clone();
        Box::pin(async move {
            let mut users = self.users.write().unwrap();
            if let Some(user) = users.get_mut(&user_id.0.to_string()) {
                user.reputation = score;
            }
            Ok(())
        })
    }

    fn register(
        &self,
        display_name: String,
        credentials: Credentials,
    ) -> BoxFuture<'_, Result<User>> {
        Box::pin(async move {
            let user = User {
                id: UserId::new(),
                display_name: display_name.clone(),
                reputation: ReputationScore::default(),
                created_at: Utc::now(),
                role: UserRole::Participant,
            };

            let mut users = self.users.write().unwrap();
            users.insert(user.id.0.to_string(), user.clone());
            drop(users);

            // Store credentials if username/password
            if let Credentials::UsernamePassword { username, password } = credentials {
                let mut creds = self.credentials.write().unwrap();
                creds.insert(username, password);
            }

            Ok(user)
        })
    }
}
