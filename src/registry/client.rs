use std::collections::HashMap;

/// Push-based client activity tracker.
///
/// The consumer provides timestamps — the crate has no clock dependency.
pub struct ClientTracker {
    clients: HashMap<u32, u64>,
}

impl ClientTracker {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Records client activity at the given timestamp.
    pub fn record_activity(&mut self, client_id: u32, timestamp: u64) {
        self.clients.insert(client_id, timestamp);
    }

    /// Returns client IDs that have not been seen since `now - timeout`.
    pub fn check_timeouts(&mut self, now: u64, timeout: u64) -> Vec<u32> {
        let threshold = now.saturating_sub(timeout);
        let timed_out: Vec<u32> = self
            .clients
            .iter()
            .filter(|(_, &last_seen)| last_seen < threshold)
            .map(|(&id, _)| id)
            .collect();

        for &id in &timed_out {
            self.clients.remove(&id);
        }

        timed_out
    }

    /// Returns whether a client is currently tracked.
    pub fn is_connected(&self, client_id: u32) -> bool {
        self.clients.contains_key(&client_id)
    }

    /// Removes a client from tracking.
    pub fn disconnect(&mut self, client_id: u32) {
        self.clients.remove(&client_id);
    }
}

impl Default for ClientTracker {
    fn default() -> Self {
        Self::new()
    }
}
