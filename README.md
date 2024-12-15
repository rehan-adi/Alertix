# Notification System

This is a Rust-based notification system consisting of:
- **Backend**: Handles incoming requests and interacts with the database.
- **Worker**: Processes queued events and delivers notifications.

## Current Structure
- `backend/`: The HTTP API service.
- `worker/`: The event processing and notification delivery service.

## Features
- Event-driven architecture with Redis queue.
- Modular and scalable design for high traffic.
