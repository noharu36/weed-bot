use std::{fs::File, io::BufReader, usize};
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json::Result;


// Handler構造体。取得したいイベントを実装する
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Botが起動したときに走る処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    //UserIdとChannelIdを取得して草を返す
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == 1006216263401488455 && msg.channel_id == 1131020631484420217{
            if let Err(why) = msg.channel_id.create_reaction(&ctx.http, msg.id, '\u{1F331}').await {
                println!("メッセージ送信エラー : {:?}", why);
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

//{"token": "This_is_Token"}
// の形のトークンを取り出す関数
fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

#[tokio::main]
async fn main() {
    // Discord Bot Token を設定
    let token = get_token("config.json").expect("Err トークンが見つかりません");
    // Botのクライアントを作成
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler) // 取得するイベント
        .await
        .expect("Err creating client"); // エラーハンドリング

    // メインループ。Botを起動
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}