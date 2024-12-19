use crate::schema::notifications;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = notifications)]
pub struct Notification {
    notification_id: Uuid,
    event_id: Uuid,
    user_id: i32,
    channel: String,
    status: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}