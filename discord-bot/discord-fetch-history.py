import requests
from interactions import Client, Intents, listen, ChannelType
from config import TOKEN, URL, SERVER_ID, CHANNEL_BLACK_LIST

bot = Client(intents=Intents.ALL, sync_interactions=True)

@listen()
async def on_ready():
  guild = bot.get_guild(SERVER_ID)

  for channel in guild.channels:    
    if channel.type != ChannelType.GUILD_TEXT: continue
    if channel.id in CHANNEL_BLACK_LIST: continue

    message_count = 0

    async for message in channel.history(limit=0):
      if len(message.content) == 0: continue
      if message.author.bot: continue
      
      message_count += 1
      result = requests.post(f'{URL}/api/markov_chain', data=message.content.encode('utf-8'))
      
      if result.content.encode('utf-8') != message.content.encode('utf-8'):
        print(result)

    print(f'{message_count} messages parsed in {channel.name}({channel.id})')
  
  print(f'Loaded all messages from {guild.name}({SERVER_ID})')
  print(f'Press ctrl+c to exit...')

bot.start(TOKEN)
