mod producer;
mod consumer;

#[tokio::main]
async fn main() {
    let producer_obj = producer::create_producer();
    producer::produce(&producer_obj, "Hello, Kafka from Rust").await;

    consumer::start_consumer().await;
}
