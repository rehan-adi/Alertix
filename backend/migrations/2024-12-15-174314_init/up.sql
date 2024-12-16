-- Create the `events` table
CREATE TABLE events (
    event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id INTEGER NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('account', 'transaction', 'loan')),
    payload JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create the `notifications` table
CREATE TABLE notifications (
    notification_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_id UUID NOT NULL REFERENCES events(event_id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL,
    channel TEXT NOT NULL CHECK (channel IN ('email', 'sms')),
    status TEXT NOT NULL CHECK (status IN ('pending', 'sent', 'failed')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
