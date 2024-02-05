use crate::models::Killmail;
use futures_util::StreamExt;
use futures::SinkExt;
use serenity::all::ChannelId;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::client::Context;
use tokio_tungstenite::connect_async;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tungstenite::Message;

pub async fn kill_feed(ctx: &Context) {
    let url = url::Url::parse("wss://zkillboard.com/websocket/").unwrap();

    let (stdin_tx, mut stdin_rx) = mpsc::unbounded_channel();
    tokio::spawn(read_stdin(stdin_tx));

    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("ZKill handshake has completed succesfully");

    let _ = ws_stream.send(Message::Text(
        "{\"action\":\"sub\",\"channel\":\"killstream\"}".into(),
    )).await;

    let (mut write, mut read) = ws_stream.split();

    let stdin_to_ws = async {
        while let Some(message) = stdin_rx.recv().await {
            write.send(message).await.expect("Failed to send message to WebSocket");
        }
    };

    let ws_to_stdout = async {
        while let Some(message) = read.next().await {
            // println!("{:?}", message);
            let data = message.unwrap().into_data();
            if !data.is_empty() {
                let parsed: Killmail = serde_json::from_slice(&data).expect("Unable to parse to json");
                
                let embed = CreateEmbed::new()
                    .title("Kill")
                    .field("Killmail", parsed.killmail_id.to_string(), false);
                let builder = CreateMessage::new().embed(embed);
                let message = ChannelId::new(586294661010685954).send_message(&ctx, builder).await;
                if let Err(why) = message {
                    eprintln!("Error sending message: {why:?}");
                };
            }
            tokio::io::stdout().write_all(&data).await.expect("Failed to write to stdout");
        }
    };

    tokio::select! {
        _ = stdin_to_ws => (),
        _ = ws_to_stdout => (),
    };
}

async fn read_stdin(tx: mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.send(Message::text(String::from_utf8(buf).unwrap())).expect("Failed to send message to WebSocket");
    }
}