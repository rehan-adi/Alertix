// @generated automatically by Diesel CLI.

diesel::table! {
    events (event_id) {
        event_id -> Uuid,
        user_id -> Int4,
        event_type -> Text,
        payload -> Jsonb,
        created_at -> Timestamp,
    }
}

diesel::table! {
    notifications (notification_id) {
        notification_id -> Uuid,
        event_id -> Uuid,
        user_id -> Int4,
        channel -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(notifications -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    notifications,
);
