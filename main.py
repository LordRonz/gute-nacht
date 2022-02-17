import os
from utils.read_env import read_env
from secrets import choice
from data.gifs import gifs
from data.quotes import quotes
from data.thumbnails import thumbnails
from data.embed_urls import embed_urls
import config
from discord_webhook.discord_webhook import DiscordWebhook
from datetime import datetime


def main():
    read_env()

    if not config.EXECUTE_TODAY:
        print("Don't forget to say it mister!")
        return

    is_ci = os.getenv("CI", "") == "true"
    webhook_url: str = os.getenv("WEBHOOK_URL_DEV" if is_ci else "WEBHOOK_URL", "")
    if is_ci and not webhook_url:
        webhook_url = os.getenv("WEBHOOK_URL", "")

    quote = choice(quotes)

    gif = choice(gifs)

    content = (
        f"**{quote}** :sleeping: :full_moon_with_face: <:jibril:914851644674572308>"
    )

    thumbnail = choice(thumbnails)

    embed_url = choice(embed_urls)

    webhook = DiscordWebhook(
        url=webhook_url,
        username=config.USERNAME,
        avatar_url=config.AVATAR_URL,
    )

    webhook.add_embed(
        {
            "title": content,
            "color": 0x631313,
            "image": {
                "url": gif,
            },
            "timestamp": datetime.now().isoformat(),
            "thumbnail": {
                "url": thumbnail,
            },
            "url": embed_url,
        }
    )

    webhook.execute()


if __name__ == "__main__":
    main()
