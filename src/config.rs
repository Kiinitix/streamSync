use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub kafka_brokers: String,
    pub topic_a: String,
    pub topic_b: String,
    pub output_topic: String,
    pub health_port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            kafka_brokers: std::env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into()),
            topic_a: std::env::var("TOPIC_A").unwrap_or_else(|_| "svcA.changes".into()),
            topic_b: std::env::var("TOPIC_B").unwrap_or_else(|_| "svcB.changes".into()),
            output_topic: std::env::var("OUTPUT_TOPIC").unwrap_or_else(|_| "reconcile.events".into()),
            health_port: std::env::var("HEALTH_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080),
        }
    }
}
