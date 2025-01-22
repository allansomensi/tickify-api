CREATE TYPE ticket_status AS ENUM ('open', 'inprogress', 'closed', 'reopened', 'paused', 'cancelled');

CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    requester UUID NOT NULL,
    status ticket_status NOT NULL,
    closed_by UUID,
    solution TEXT,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    closed_at TIMESTAMP NULL,
    FOREIGN KEY (requester) REFERENCES users(id),
    FOREIGN KEY (closed_by) REFERENCES users(id)
);
