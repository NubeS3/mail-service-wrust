use tokio::task;

fn main() {
    const NATS_URL: &str = "http://localhost:4222";
    const MAIL_SUBJ: &str = "nubes3_mail";

    let nc = nats::connect(NATS_URL)?;
    let sub = nc.subscribe(MAIL_SUBJ)?;
    task::spawn(async {
        if let Some(data) = sub.next().await? {
            let msg = json!(data);
            //Send mail func
        }
    });
}


