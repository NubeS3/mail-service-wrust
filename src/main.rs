extern crate dotenv;

use dotenv::dotenv;
use nats;
use std::env;
use tokio::task;

fn main() -> std::io::Result<()> {
  dotenv().ok();

  let nats_url = env::var("NATS_URL").ok().unwrap();
  let nats_sbj = env::var("NATS_SBJ").ok().unwrap();
  let nats_queue = env::var("NATS_QUEUE").ok().unwrap();

  let nats = nats::connect(&nats_url).ok().unwrap();

  for _ in 0..12 {
    let nats1 = nats.clone();
    let nats_sbj1 = nats_sbj.clone();
    let nats_queue1 = nats_queue.clone();
    task::spawn(async {
      handler(nats1, nats_sbj1, nats_queue1).await;
    });
  }
  Ok(())
}

async fn handler(nc: nats::Connection, nats_sbj: String, nats_queue: String) {
  let sub = nc.queue_subscribe(&nats_sbj, &nats_queue).ok().unwrap();
  loop {
    if let Some(msg) = sub.next() {
      let msg = String::from_utf8(msg.data.clone()).unwrap();
      println!("{}", msg);
    }
  }
}
