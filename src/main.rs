mod config;
mod discord;
mod leetcode;

use config::Config;
use discord::DiscordClient;

#[tokio::main]
async fn main() {
    let config = Config::load();
    let token = config.get_discord_token().unwrap();
    let stream_chat_id = config.get_channel_id().unwrap();

    let discord_client = DiscordClient::build(token, stream_chat_id).await;

    let daily = leetcode::get_daily_leetcode().await.unwrap();

    discord_client.send_leetcode(daily).await;
}
