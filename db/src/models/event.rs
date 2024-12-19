use crate::schema::events;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = events)]
pub struct Event {
    pub event_id: Uuid,
    pub user_id: i32,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
}
