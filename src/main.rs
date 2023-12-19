use teloxide::{prelude::*, utils::command::BotCommands};
use std::env;
use dotenv::dotenv;
use reqwest;
use serde_json;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bot = Bot::new(env::var("TELEGRAM_API").unwrap());

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "fetch price of provided token in USD and EUR.")]
    Price(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Price(token) => {
            let _msg = bot.send_message(msg.chat.id, "...").await?;
            let result = fetch_token_price(token.as_str()).await?;
            bot.delete_message(msg.chat.id, _msg.id).await?;
            bot.send_message(msg.chat.id, result).await?
        }
    };

    Ok(())
}

async fn fetch_token_price(token: &str) -> Result<String, reqwest::Error> {
    println!("Fetching https://api.coingecko.com/api/v3/coins/{}?localization=false", token);

    let url = format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", token.to_lowercase());
    let response = reqwest::get(url).await?.text().await?;
    let json: serde_json::Value = serde_json::from_str(&response).unwrap();

    let result = format!(
        "Price of {token} is ${price_usd} or €{price_eur}",
        token=token,
        price_usd=json["market_data"]["current_price"]["usd"],
        price_eur=json["market_data"]["current_price"]["eur"]);

    Ok(result)
}