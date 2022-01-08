use std::collections::HashMap;
use uuid::Uuid;
use crate::OnEvent;
use crate::system::user::{UserCreated};

#[derive(Default)]
pub struct TradeExecutor {
    pub user_keys: HashMap<Uuid, String>
}

impl OnEvent<UserCreated> for TradeExecutor {
    fn on_event(&mut self, event: UserCreated) {
        self.user_keys.insert(event.id, event.api_key);
    }
}