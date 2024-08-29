use rdkafka::ClientConfig;
use rdkafka::producer::{ FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

pub fn create_producer() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

    let producer: FutureProducer = config
    .create()
    .expect("Producer creation error");

    producer
}

pub async fn produce(future_producer: &FutureProducer, message: &str) {
    let record = FutureRecord::to("test_topic")
    .payload(message)
    .key("test_key");

    let status_delivery = future_producer.send(
        record, 
        Timeout::After(Duration::from_secs(2))
    ).await;

    match status_delivery {
        Ok(delivery_status) => println!("Message send successfully. Report: {:?}", delivery_status),
        Err(e) => println!("Error sending message: {:?}", e)
    }
}