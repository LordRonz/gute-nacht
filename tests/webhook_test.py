from discord_webhook.discord_webhook import DiscordWebhook
from utils.read_env import read_env
from os import getenv

read_env()


def set_webhook():
    webhook = DiscordWebhook(url=getenv("WEBHOOK_URL_DEV", ""), content="TEST")
    webhook.add_embed({"title": "Test Embed Title"})
    return webhook


def test_webhook_content():
    webhook = set_webhook()

    assert webhook.content == "TEST"


def test_webhook_embed():
    webhook = set_webhook()

    assert webhook.get_embeds()[0]["title"] == "Test Embed Title"
