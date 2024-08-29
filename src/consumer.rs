use rdkafka::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::consumer::CommitMode;
use rdkafka::consumer::Consumer;
use rdkafka::Message;


pub async fn start_consumer() {
    let consumer: StreamConsumer = create_consumer();
    consume(consumer).await
}

pub fn create_consumer() -> StreamConsumer {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", "kafka:9092")
        .set("group.id", "test_group")
        .set("auto.offset.reset", "earliest")
        .set("socket.timeout.ms", "4000").create().expect("Consumer creation error");
    // let consumer = config.create().expect("Consumer creation error");
    consumer
}

async fn consume(consumer: StreamConsumer) {
    consumer.subscribe(&["test_topic"]).expect("Unable to subscribe to test_topic");
    loop { //we want to keep the consumer running until it's manually stopped
        match consumer.recv().await {
            Err(e) => println!("Error receiving message: {:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    Some(Err(e)) => println!("Error getting payload: {:?}", e),
                    Some(Ok(msg_payload)) => println!("Received message: {:?}", msg_payload),
                    None => println!("Message is empty"),
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
        }
    }
}