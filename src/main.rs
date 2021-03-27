use std::env;

use dotenv::dotenv;
use sendgrid::{Destination, Mail, SGClient};
use serde::Deserialize;
use serde_json;
use tokio::task;

#[derive(Deserialize)]
struct MsgInfo {
  username: String,
  to: String,
  otp: String,
  exp: String,
}

fn main() {
  dotenv().ok();

  let nats_endpoint = match env::var("NATS_ENDPOINT") {
    Ok(endpoint) => endpoint,
    Err(_) => panic!("endpoint invalid"),
  };

  let nats = match nats::connect(nats_endpoint.as_str()) {
    Ok(c) => c,
    Err(_) => panic!("nats connect failed"),
  };

  let sbj = match env::var("NATS_SBJ") {
    Ok(s) => s,
    Err(_) => panic!("subject invalid"),
  };

  let sub = match nats.queue_subscribe(&sbj, "q") {
    Ok(s) => s,
    Err(_) => panic!("subcribe queue failed"),
  };

  let sg_key = match env::var("SG_API_KEY") {
    Ok(key) => key,
    Err(_) => panic!("Invalid sendgrid api key"),
  };

  let sg = SGClient::new(sg_key);

  loop {
    if let Some(msg) = sub.next() {
      let sg1 = sg.clone();
      task::spawn(async move {
        let msg_info: MsgInfo = match serde_json::from_slice(&msg.data) {
          Ok(i) => i,
          Err(_) => panic!("parse info from message failed"),
        };
        let mail = Mail::new()
          .add_to(Destination {
            address: msg_info.to.as_str(),
            name: msg_info.username.as_str(),
          })
          .add_from("nubes3cloud@gmail.com")
          .add_subject("Verify NubeS3 account")
          .add_from_name("NubeS3 Team")
          .add_content(
            format!(
              "Do not share this OTP to others./n
              OTP: {}.\n
              Expired time: {}",
              msg_info.otp, msg_info.exp
            ),
            "text/html",
          );
        match sg1.send(mail) {
          Ok(resp) => println!("Response: {:?}", resp),
          Err(e) => println!("Err: {}", e),
        };
      });
    }
  }
}
