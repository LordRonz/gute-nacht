from data.embed_urls import embed_urls
from data.emojis import emojis
from data.gifs import gifs
from data.quotes import quotes
from data.thumbnails import thumbnails


def test_embed_urls():
    assert len(set(embed_urls)) == len(embed_urls)


def test_emojis():
    for i in range(len(emojis) - 1):
        for j in range(i + 1, len(emojis)):
            assert emojis[i] != emojis[j]


def test_gifs():
    assert len(set(gifs)) == len(gifs)


def test_quotes():
    assert len(set(quotes)) == len(quotes)


def test_thumbnails():
    assert len(set(thumbnails)) == len(thumbnails)
