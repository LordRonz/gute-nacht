use data::description;
use data::description::get_description;
use data::embed_urls;
use data::embed_urls::get_embed_urls;
use data::thumbnails::get_thumbnails;
use dotenv::dotenv;
mod data;
mod discord_webhook;
use data::gifs::*;
use data::quotes::*;

use crate::data::emojis::get_emojis;

fn main() {
    dotenv().ok();

    let is_ci = std::env::var("CI").unwrap_or("".into()) == "true";

    let webhook_url_env_key = if is_ci {
        "WEBHOOK_URL_DEV"
    } else {
        "WEBHOOK_URL"
    };

    let webhook_url = std::env::var(webhook_url_env_key).expect("WEBHOOK_URL must be set.");

    let quotes = get_quotes();

    let gif = get_gifs();

    let thumbnail = get_thumbnails();

    let embed_url = get_embed_urls();

    for quote in quotes.iter() {
        println!("{}", quote)
    }

    let description = get_description();

    println!("{}", description);
}
