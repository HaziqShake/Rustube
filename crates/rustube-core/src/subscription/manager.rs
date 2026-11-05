use crate::storage::database::{Database, DatabaseError};

pub struct SubscriptionManager {
    db: Database,
}

impl SubscriptionManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn subscribe(&self, id: &str, name: &str, avatar: Option<&str>) -> Result<(), DatabaseError> {
        self.db.add_subscription(id, name, avatar)
    }

    pub fn unsubscribe(&self, id: &str) -> Result<(), DatabaseError> {
        self.db.remove_subscription(id)
    }

    pub fn is_subscribed(&self, id: &str) -> Result<bool, DatabaseError> {
        self.db.is_subscribed(id)
    }

    pub fn get_subscriptions(&self) -> Result<Vec<(String, String, Option<String>)>, DatabaseError> {
        self.db.get_subscriptions()
    }
}
