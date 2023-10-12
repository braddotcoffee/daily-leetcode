use crate::leetcode;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::utils::Color;

const GREEN_COLOR_CODE: i32 = 0x3FCA7D;
const YELLOW_COLOR_CODE: i32 = 0xFFC926;
const RED_COLOR_CODE: i32 = 0xFF5967;

pub struct DiscordClient {
    pub client: Http,
    pub channel_id: u64,
}

impl DiscordClient {
    pub async fn build(token: String, channel_id: u64) -> DiscordClient {
        let client = Http::new(&token);
        DiscordClient { client, channel_id }
    }

    pub async fn send_leetcode(&self, problem: leetcode::LeetCodeQuestion) {
        let color = match problem.difficulty.as_str() {
            "Easy" => Color::from(GREEN_COLOR_CODE),
            "Medium" => Color::from(YELLOW_COLOR_CODE),
            "Hard" => Color::from(RED_COLOR_CODE),
            _ => Color::BLURPLE,
        };

        let channel = ChannelId::from(self.channel_id);

        let response = channel
            .send_message(&self.client, |m| {
                m.embed(|e| {
                    e.color(color)
                        .title(format!("Daily LeetCode! ({})", problem.difficulty))
                        .description(problem.title)
                        .field("Link", problem.link, false)
                })
            })
            .await;
        if let Ok(message) = response {
            let _ = channel
                .create_public_thread(&self.client, message.id, |m| {
                    m.name("Post your answers here!")
                })
                .await;
        }
    }
}
