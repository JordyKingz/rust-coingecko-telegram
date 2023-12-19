## Telegram Rust Bot
Want quick access to Crypto prices? This bot will give you the current price of any cryptocurrency you ask for. Using the CoinGecko API, this bot will give you the current price of any cryptocurrency you ask for.

### Setting up your environment
1. Download Rust.
2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format 123456789:blablabla.
3. Add TELEGRAM_API environment variable to your `.env` file.

### Running the bot
1. Clone this repo.
2. Run `cargo run` in the root directory.
3. Open Telegram and search for your bot.
4. Send a message to your bot in the format `/price {crypto}`. For example, `/price bitcoin`.
5. You should get a response with the current price of the crypto you asked for.