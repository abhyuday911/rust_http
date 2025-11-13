use tokio::sync::mpsc;

pub async fn run_engine(mut receiver: mpsc::Receiver<String>) {
    while let Some(i) = receiver.recv().await {
        println!("got some value i.e. {}", i)
    }
}
