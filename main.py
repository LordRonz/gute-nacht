import requests, os
from utils.read_env import read_env
from random import choice
from data.gifs import gifs
from data.quotes import quotes
from utils.logger import get_logger
import config


def main():
    read_env()

    WEBHOOK_URL: str = os.getenv("WEBHOOK_URL", "")

    logger = get_logger()

    if not WEBHOOK_URL:
        logger.error("Webhook url cannot be empty!")
        return

    quote = choice(quotes)

    gif = choice(gifs)

    payload = {
        "username": config.USERNAME,
        "avatar_url": config.AVATAR_URL,
        "content": f"**{quote}** :sleeping: :full_moon_with_face:\n{gif}",
    }

    logger.info(payload)

    res = requests.post(WEBHOOK_URL, json=payload)

    logger.info(f"http response status: {res.status_code}")


if __name__ == "__main__":
    main()
