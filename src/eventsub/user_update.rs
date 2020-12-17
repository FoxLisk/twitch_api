use super::*;
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserUpdate {
    pub user_id: types::UserId,
}

impl EventSubscription for UserUpdate {
    type Payload = UserUpdatePayload;

    const EVENT_TYPE: EventType = EventType::UserUpdate;
    const VERSION: &'static str = "1";

    fn condition(&self) -> Result<serde_json::Value, serde_json::Error> { todo!() }
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct UserUpdatePayload {}

impl NotificationPayload for UserUpdatePayload {}
