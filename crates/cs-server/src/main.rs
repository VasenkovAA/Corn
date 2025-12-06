use cs_protocol::HelloMessage;

fn main() {
    let msg = HelloMessage {
        text: "Hello from Server!".to_string(),
    };

    let json = serde_json::to_string(&msg).unwrap();

    println!("Наш сервер создал сообщение:");
    println!("{}", json);
}
