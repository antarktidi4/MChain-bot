import requests
from interactions import Client, Intents, listen
from config import TOKEN, URL, MAIN_CHANNEL_ID

bot = Client(intents=Intents.ALL, sync_interactions=True)

@listen()
async def on_ready():
  print('Ready')

  async for message in bot.get_channel(MAIN_CHANNEL_ID).history(limit=0):
    if len(message.content) == 0: continue
    if message.author.bot: continue
    result = requests.post(f'{URL}/api/markov_chain', data=message.content.encode('utf-8'))
    if result.content.encode('utf-8') != message.content.encode('utf-8'):
      print(result)
    else:
      print(f'Success loaded message {message.id}')
  
  print(f'Loaded all messages from main channel({MAIN_CHANNEL_ID})')


bot.start(TOKEN)