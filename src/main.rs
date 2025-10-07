mod config;
mod reconciler;
mod state_store;

use anyhow::Result;
use futures::StreamExt;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::task;
use warp::Filter;

use crate::config::AppConfig;
use crate::reconciler::Reconciler;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let cfg = AppConfig::default();
    let reconciler = Arc::new(Mutex::new(Reconciler::new()));

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.kafka_brokers)
        .set("group.id", "rapidreconcile-group")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&[&cfg.topic_a, &cfg.topic_b])?;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.kafka_brokers)
        .create()?;

    let reconciler_consumer = reconciler.clone();
    let producer_clone = producer.clone();
    let topic_out = cfg.output_topic.clone();

    let consumer_task = task::spawn(async move {
        let mut message_stream = consumer.stream();
        while let Some(result) = message_stream.next().await {
            match result {
                Ok(borrowed_msg) => {
                    if let Err(e) = handle_message(&borrowed_msg, &reconciler_consumer, &producer_clone, &topic_out).await {
                        log::error!("handle_message error: {:?}", e);
                    }
                    if let Err(e) = consumer.commit_message(&borrowed_msg, CommitMode::Async) {
                        log::warn!("commit error: {:?}", e);
                    }
                }
                Err(e) => {
                    log::error!("Kafka error: {:?}", e);
                }
            }
        }
    });

    let health = warp::path!("health").map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK));
    let server = warp::serve(health).run(([0, 0, 0, 0], cfg.health_port));

    tokio::select! {
        _ = consumer_task => {},
        _ = server => {},
    }

    Ok(())
}

async fn handle_message(
    msg: &BorrowedMessage<'_>,
    reconciler: &Arc<Mutex<Reconciler>>,
    producer: &FutureProducer,
    output_topic: &str,
) -> Result<()> {
    let payload = match msg.payload_view::<str>() {
        None => {
            log::warn!("empty payload");
            return Ok(());
        }
        Some(Ok(s)) => s.to_string(),
        Some(Err(_)) => {
            log::warn!("invalid payload encoding");
            return Ok(());
        }
    };

    let json: Value = serde_json::from_str(&payload)?;
    let topic = msg.topic().to_string();

    {
        let mut r = reconciler.lock().unwrap();
        r.ingest(&topic, json.clone());
    }

    let maybe_result = {
        let r = reconciler.lock().unwrap();
        r.try_reconcile_for_message(&topic, &json)
    };

    if let Some(res_json) = maybe_result {
        let payload = serde_json::to_string(&res_json)?;
        let key_str: String = res_json
            .get("entity_id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let record: FutureRecord<String, String> = FutureRecord::to(output_topic)
            .payload(&payload)
            .key(&key_str);

        let delivery = producer.send(record, Duration::from_secs(0)).await;

        match delivery {
            Ok((_partition, _offset)) => {
                log::info!("Produced reconcile event for {}", key_str);
            }
            Err((e, _owned_msg)) => {
                log::error!("Produce error: {:?}", e);
            }
        }
    }

    Ok(())
}
