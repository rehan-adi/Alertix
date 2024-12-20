use crate::redis::redis_client;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

#[derive(Deserialize, Serialize, Debug)]
struct Event {
    event_id: String,
    user_id: i64,
    event_type: String,
    payload: Value,
    created_at: String,
}

#[derive(Deserialize, Debug)]
struct LoanPayload {
    channel: String,
    loan_id: String,
    loan_amount: String,
    status: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct AccountPayload {
    channel: String,
    name: String,
    email: String,
    phone: String,
    message: String,
}

#[derive(Deserialize, Debug)]
struct TransactionPayload {
    channel: String,
    transaction_id: String,
    amount: String,
    transaction_type: String,
    balance: String,
    message: String,
}

pub async fn process_redis_queue() {
    let mut con = redis_client().await;

    println!("Worker started. Waiting for events...");
    let key = String::from("event_queue");

    loop {
        match con
            .brpop::<String, (String, String)>(key.clone(), 0.0)
            .await
        {
            Ok((_, event_data)) => {
                match serde_json::from_str::<Event>(&event_data) {
                    Ok(event) => {
                        println!(
                            "Processing event: {}, User ID: {}, Event Type: {}, Created At: {}",
                            event.event_id, event.user_id, event.event_type, event.created_at
                        );

                        // Check event type and process accordingly
                        match event.event_type.as_str() {
                            "loan" => {
                                if let Ok(loan_payload) =
                                    serde_json::from_value::<LoanPayload>(event.payload)
                                {
                                    println!(
                                        "Processing loan event. Channel: {}, Loan ID: {}, Amount: {}, Status: {}, Message: {}",
                                        loan_payload.channel,
                                        loan_payload.loan_id,
                                        loan_payload.loan_amount,
                                        loan_payload.status,
                                        loan_payload.message
                                    );
                                    // Add your loan-specific processing logic here
                                } else {
                                    eprintln!("Failed to deserialize loan payload.");
                                }
                            }
                            "transaction" => {
                                if let Ok(transaction_payload) =
                                    serde_json::from_value::<TransactionPayload>(event.payload)
                                {
                                    println!(
                                        "Processing transaction event. Channel: {}, Transaction ID: {}, Amount: {}, Type: {}, Balance: {}, Message: {}",
                                        transaction_payload.channel,
                                        transaction_payload.transaction_id,
                                        transaction_payload.amount,
                                        transaction_payload.transaction_type,
                                        transaction_payload.balance,
                                        transaction_payload.message
                                    );
                                    // Add your transaction-specific processing logic here
                                } else {
                                    eprintln!("Failed to deserialize transaction payload.");
                                }
                            }
                            "account" => {
                                if let Ok(account_payload) =
                                    serde_json::from_value::<AccountPayload>(event.payload)
                                {
                                    println!(
                                        "Processing account event. Channel: {}, Name: {}, Email: {}, Phone: {},
                                        Message: {}",
                                        account_payload.channel,
                                        account_payload.name,
                                        account_payload.email,
                                        account_payload.phone,
                                        account_payload.message
                                    );
                                    // Add your account-specific processing logic here
                                } else {
                                    eprintln!("Failed to deserialize account payload.");
                                }
                            }
                            _ => {
                                eprintln!("Unknown event type: {}", event.event_type);
                            }
                        }

                        // Simulate processing time
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                        println!("Event processed successfully!");
                    }
                    Err(e) => {
                        eprintln!("Failed to deserialize event: {}", e);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to fetch event: {}", err);
            }
        }
    }
}
