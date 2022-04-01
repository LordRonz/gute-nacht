import os
from utils.read_env import read_env
from secrets import choice
from data.gifs import gifs
from data.quotes import get_quote
from data.thumbnails import thumbnails
from data.embed_urls import embed_urls
from data.emojis import emojis
import config
from discord_webhook.discord_webhook import DiscordWebhook
from datetime import datetime
from base64 import b64decode


def main():
    read_env()

    if not config.EXECUTE_TODAY:
        print("Don't forget to say it mister!")
        return

    is_ci = os.getenv("CI", "") == "true"
    webhook_url: str = os.getenv("WEBHOOK_URL_DEV" if is_ci else "WEBHOOK_URL", "")
    if is_ci and not webhook_url:
        webhook_url = os.getenv("WEBHOOK_URL", "")

    quote = get_quote()

    gif = choice(gifs)

    emoji = choice(emojis)

    title = f"**{quote}** :sleeping: :full_moon_with_face: <{'a' if emoji['animated'] else ''}:{emoji['name']}:{emoji['id']}>"

    description_list = []
    for _ in range(3):
        emoji = choice(emojis)
        description_list.append(
            f"<{'a' if emoji['animated'] else ''}:{emoji['name']}:{emoji['id']}>"
        )

    description = " ".join(description_list)

    tsuki = choice([0] * 35 + [1] * 30 + [2] * 20 + [3] * 10 + [4] * 5)
    if tsuki == 1:
        description += (
            f"\nP.S. : ||{b64decode('VHN1a2kgZ2Ega2lyZWkgZGVzdSBuZT8=').decode()}||"
        )
    elif tsuki == 2:
        description += f"\nP.S. : ||{b64decode('QWlzaGl0ZXJ1').decode()}||"
    elif tsuki == 3:
        description += f"\nP.S. : ||{b64decode('UmVtaW5kZXI6IEFrdSBzYXlhbmdnZyBrYW11IDwz').decode()}||"
    elif tsuki == 4:
        description += f"\nP.S. : ||{b64decode('SSB3aWxsIGFsd2F5cyBsb3ZlIHlvdSBhbmQgYmUgYnkgeW91ciBzaWRl').decode()}||"

    pps = choice([True, True, False])
    if pps and tsuki:
        description += f"\nP.P.S : ||{b64decode('SSByZWFsbHkgbWVhbiBpdA==').decode()}||"

    thumbnail = choice(thumbnails)

    embed_url = choice(embed_urls)

    webhook = DiscordWebhook(
        url=webhook_url,
        username=config.USERNAME,
        avatar_url=config.AVATAR_URL,
    )

    webhook.add_embed(
        {
            "title": title,
            "description": description,
            "color": 0x631313,
            "image": {
                "url": gif,
            },
            "timestamp": datetime.now().isoformat(),
            "thumbnail": {
                "url": thumbnail,
            },
            "url": embed_url,
            "footer": {
                "text": "Sincerely, A",
                "icon_url": "https://www.freeiconspng.com/uploads/love-png-5.png",
            },
        }
    )

    webhook.execute()


if __name__ == "__main__":
    main()
