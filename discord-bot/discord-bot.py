import requests, random
from interactions import Client, Intents, listen, slash_command, SlashContext
from interactions.ext import prefixed_commands
from config import TOKEN, URL, MAIN_CHANNEL_ID

bot = Client(intents=Intents.ALL, sync_interactions=True)
prefixed_commands.setup(bot)

@listen()
async def on_ready():
  print('Ready')

@listen()
async def on_message_create(event):
  if event.message.author.bot: return

  if random.randint(0, 10) == 1:
    await event.message.channel.send(event.message.content)

  if len(event.message.content) == 0: return

  requests.post(f'{URL}/api/markov_chain', data=event.message.content.encode('utf-8'))
  print(f'Message received and added: `{event.message.content}`')

@slash_command(name='generate', description='Generates a text by markov chain algorythm')
async def generate(ctx: SlashContext):
  generated_text = requests.get(f'{URL}/api/markov_chain').content.decode('utf-8') 
  print(f'Generated: {generated_text}')
  await ctx.send(generated_text)


bot.start(TOKEN)