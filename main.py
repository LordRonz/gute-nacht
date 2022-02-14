import requests, os
from utils.read_env import read_env
from random import choice
from data.gifs import gifs
from data.quotes import quotes
import config
from discord_webhook.discord_webhook import DiscordWebhook


def main():
    read_env()
    is_ci = os.getenv("CI", "") == "true"
    webhook_url: str = os.getenv("WEBHOOK_URL_DEV" if is_ci else "WEBHOOK_URL", "")
    if is_ci and not webhook_url:
        webhook_url = os.getenv("WEBHOOK_URL", "")

    quote = choice(quotes)

    gif = choice(gifs)

    content = f"**{quote}** :sleeping: :full_moon_with_face:\n{gif}"

    webhook = DiscordWebhook(
        url=webhook_url,
        content=content,
        username=config.USERNAME,
        avatar_url=config.AVATAR_URL,
    )

    webhook.execute()


if __name__ == "__main__":
    main()
