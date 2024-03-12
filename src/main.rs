use tokio::sync::watch;
use tokio::sync::watch::{Receiver, Sender};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let (tx, rx): (Sender<&str>, Receiver<&str>) = watch::channel("");

    // Create handlers
    let mut handlers = Vec::new();

    // create receivers
    for i in 0..5 {
        let rx = rx.clone();
        let handler = tokio::spawn(async move {
            loop {
                let msg: &str = rx.borrow().clone();

                if msg.is_empty() {
                    continue;
                }

                println!("Routine {num} received message {msg}.", num = i);

                return;
            }
        });

        handlers.push(handler);
    }

    tx.send("hello world").unwrap();

    for task in handlers {
        task.await.expect("task failed");
    }
}
