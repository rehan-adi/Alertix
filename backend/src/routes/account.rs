use crate::config::redis::redis_client;
use crate::types::account::AccountData;
use crate::{config::db::DbPool, models::event::Event, schema::events};
use actix_web::{post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use log::error;
use redis::AsyncCommands;
use serde_json::json;
use uuid::Uuid;

#[post("/account")]
pub async fn account(pool: web::Data<DbPool>, data: web::Json<AccountData>) -> impl Responder {
    let conn = &mut pool
        .get()
        .expect("Failed to get a connection from the pool");

    // Convert event_id and user_id to appropriate types
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

    // Create the payload as a JSON object
    let payload = json!({
        "name": data.name,
        "email": data.email,
        "phone": data.phone,
        "message": data.message
    });

    // Parse the created_at string into NaiveDateTime
    let created_at = match NaiveDateTime::parse_from_str(&data.created_at, "%Y-%m-%dT%H:%M:%S") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid created_at format"
            }))
        }
    };

    // Create the Event struct
    let new_event = Event {
        event_id,
        user_id,
        event_type: format!("{:?}", data.event_type).to_lowercase(),
        payload,
        created_at,
    };

    // Insert the event into the database
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
