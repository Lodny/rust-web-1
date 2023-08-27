//! Simplistic Model Layer
//! (with mock-store layer)

use serde::{Deserialize, Serialize};

// region:      -- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    id: u64,
    title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    title: String,
}


// endregion:   -- Ticket Types

