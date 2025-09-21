use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

pub struct DiscordPresence<'a> {
    client: DiscordIpcClient,
    state: &'a str,
    details: &'a str,
    connected: bool,
}
impl<'a> DiscordPresence<'a> {
    pub fn new(client_id: String, state: &'a str, details: &'a str) -> Self {
        let mut client = DiscordIpcClient::new(&client_id);
        let connected = match client.connect() {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Failed to connect to Discord: {}", e);
                false
            }
        };
        DiscordPresence {
            client,
            state,
            details,
            connected,
        }
    }
    pub fn update_activity(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.connected {
            return Ok(()); // Skip updating activity if not connected
        }

        let payload = Activity::new().state(self.state).details(self.details);
        match self.client.set_activity(payload) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to update Discord activity: {}", e);
                self.connected = false; // Mark as disconnected on error
                Ok(()) // Return Ok to prevent program from crashing
            }
        }
    }
}
