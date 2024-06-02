pub struct DiscordWebhook {
    url: String,
    content: String,
    username: String,
    avatar_url: String,
    tts: String,
    files: String,
    embeds: Vec<Embed>,
    proxies: String,
    allowed_mentions: String,
    timeout: i32,
    rate_limit_retry: i32,
}

struct EmbedImage {
    url: String,
}

struct EmbedFooter {
    text: String,
    icon_url: String,
}

struct Embed {
    title: String,
    description: String,
    color: String,
    image: EmbedImage,
    timestamp: String,
    thumbnail: EmbedImage,
    embed_url: String,
    footer: EmbedFooter,
}

impl DiscordWebhook {
    fn execute() {
        
    }
}
