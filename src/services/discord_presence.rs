use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

pub struct DiscordPresence<'a> {
    client: DiscordIpcClient,
    state: &'a str,
    details: &'a str,
}
impl<'a> DiscordPresence<'a> {
    pub fn new(client_id: String, state: &'a str, details: &'a str) -> Self {
        let mut client = DiscordIpcClient::new(&client_id);
        client.connect().unwrap();
        DiscordPresence {
            client,
            state,
            details,
        }
    }
    pub fn update_activity(self: &mut Self) -> Result<(), Box<dyn std::error::Error>> {
        let payload = Activity::new().state(self.state).details(self.details);
        self.client.set_activity(payload)?;
        Ok(())
    }
}
