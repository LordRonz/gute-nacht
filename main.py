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
    WEBHOOK_URL: str = os.getenv("WEBHOOK_URL_DEV" if is_ci else "WEBHOOK_URL", "")

    quote = choice(quotes)

    gif = choice(gifs)

    content = f"**{quote}** :sleeping: :full_moon_with_face:\n{gif}"

    webhook = DiscordWebhook(
        url=WEBHOOK_URL,
        content=content,
        username=config.USERNAME,
        avatar_url=config.AVATAR_URL,
    )

    webhook.execute()

    # payload = {
    #     "username": config.USERNAME,
    #     "avatar_url": config.AVATAR_URL,
    #     "content": f"**{quote}** :sleeping: :full_moon_with_face:\n{gif}",
    # }

    # logger.info(payload)

    # res = requests.post(WEBHOOK_URL, json=payload)

    # logger.info(f"http response status: {res.status_code}")


if __name__ == "__main__":
    main()
