import os
from modules import on_start, _bot
from utils import jsonIO
from pathlib import Path
DIR = Path(__file__).parent.parent.resolve()

__all__ = ["DIR", "bot_config", "save_bot_config", "data"]

if not os.path.exists(f"{DIR}/data/config.json"):
    jsonIO.dump(f"{DIR}/data/config.json", {"command_prefix": "%", "bot_admins": []})
bot_config: dict = jsonIO.load(f"{DIR}/data/config.json")
def save_bot_config():
    jsonIO.dump(f"{DIR}/data/config.json", bot_config)

data = jsonIO.load(f"{DIR}/bot_info.json")

bot = _bot.BOT()