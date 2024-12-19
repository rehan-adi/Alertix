use crate::config::redis::redis_client;
use crate::types::types::TransactionData;
use crate::{db::db::DbPool, db::models::event::Event, db::schema::events};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use log::error;
use redis::AsyncCommands;
use serde_json::json;
use uuid::Uuid;

#[post("/transaction")]
pub async fn transaction(
    pool: web::Data<DbPool>,
    data: web::Json<TransactionData>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get database pool");

    let event_id = match Uuid::parse_str(&data.event_id) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid event_id format"
            }))
        }
    };

    let user_id = match data.user_id.parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid user_id format"
            }))
        }
    };

    let payload = json!({
       "email": &data.email,
       "phone": &data.phone,
       "transaction_id": &data.transaction_id,
       "amount": &data.amount,
       "transaction_type": &data.transaction_type,
       "balance": &data.balance,
       "message": &data.message,
    });

    let created_at = match NaiveDateTime::parse_from_str(&data.created_at, "%Y-%m-%dT%H:%M:%S") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid created_at format"
            }))
        }
    };

    let new_event = Event {
        event_id,
        user_id,
        event_type: format!("{:?}", data.event_type).to_lowercase(),
        payload,
        created_at,
    };

    match diesel::insert_into(events::table)
        .values(&new_event)
        .execute(conn)
    {
        Ok(_) => {
            let event =
                serde_json::to_string(&new_event).expect("Failed to serialize event to JSON");
            let mut redis_conn = redis_client().await;

            if let Err(err) = redis_conn
                .lpush::<&str, String, ()>("event_queue", event)
                .await
            {
                error!("Failed to add event to Redis queue: {}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to add event to Redis queue"
                }));
            }

            HttpResponse::Ok().json(json!({
                "message": "Data added to database and Redis queue"
            }))
        }
        Err(e) => {
            error!("Failed to insert event: {}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to add data to database",
                "details": e.to_string()
            }))
        }
    }
}
