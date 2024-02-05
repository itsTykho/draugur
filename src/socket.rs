use crate::models::Killmail;
use serenity::client::Context;
use tungstenite::{connect, Message};

pub async fn start_socket(ctx: &Context) {
    // env_logger::init();

    let (mut socket, response) = connect("wss://zkillboard.com/websocket/").unwrap();

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .send(Message::Text(
            "{\"action\":\"sub\",\"channel\":\"killstream\"}".into(),
        ))
        .unwrap();
    loop {
        socket
            .send(Message::Text("thanks".into()))
            .unwrap();
        let msg = socket.read().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            error => error.to_string(),
        };

        if !msg.is_empty() {
            let parsed: Killmail = serde_json::from_str(&msg).expect("Cant parse to json");
            // let _ = parsed.clone().insert_killmail(&pool).await;
            print!("{:#?}", parsed);
        } else {
            continue;
        }
    }
}