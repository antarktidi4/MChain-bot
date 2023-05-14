# MChain-bot
Markov Chain Discord Bot.

# Features
- data set based on your server messages
- has a 10% chance of sending the generated text on any message
- `/generate` slash command generates text

# Installation
1. Clone repo
2. Build server
    ```bash
    cd markov-chain-api
    cargo build --release
    ```
3. Copy the server into the bot path 
4. Install modules for the discord bot
    ```bash
    cd discord-bot
    python3 -m pip install requirements.txt
    ```
5. Run server\
    default `<host>:<port>` is `127.0.0.1:8080`
    ```bash
    markov-chain-api.<ext> <host>:<port>
    ```
6. Configure discord bot\
    edit `config.py` in your favorite text editor or IDE
    ```python
    # config.py
    TOKEN = "token" # Your discord bot token
    URL = "https://<host>:<port?>" # Server url
    SERVER_ID = 755679434585 # Your discord server id
    CHANNEL_BLACK_LIST = [625237354678, 154572567, 9564568478] # Channel IDs from wich data will not be collected
    ```
7. **ONLY ON FIRST RUN**
    ```bash
    python3 discord-fetch-history.py
    ```
8. Run bot
    ```bash
    python3 discord-bot.py
    ```

# Problems
- Dataset have no weights, so it grow fast af (weights implemented by repeatedly adding a word to the next word list, so random have more chance to choose a more relevant word)
- Words don't convert to the initial form