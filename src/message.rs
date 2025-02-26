#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Message<M> {
    pub message: M,               // actual message
    pub timestamp: DateTime<Utc>, // time the message was sent
}

impl<M: PartialEq> PartialEq for Message<M> {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl<M: Eq> Eq for Message<M> {}

impl<M: Hash> Hash for Message<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.message.hash(state);
    }
}

impl<M> Message<M> {
    pub fn new(message: M) -> Self {
        Self {
            message,
            timestamp: Utc::now(),
        }
    }
}
