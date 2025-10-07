# streamSync: Real-Time Data Consistency Checker

streamSync is a lightweight Rust-based microservice that checks data consistency between two Kafka topics in real time.  
It consumes messages from two input topics, compares them by a shared key (for example, `entity_id`), and publishes reconciliation results (match or drift) to an output topic.

## Features
- Written in Rust for high performance
- Consumes and compares Kafka streams in real time
- Detects and reports data mismatches across systems
- Publishes reconciliation results to a Kafka topic
- Includes Docker setup for quick local deployment

## Architecture
<img width="1624" height="622" alt="image" src="https://github.com/user-attachments/assets/3986b203-f6b0-4b3b-9df3-bcd07d0617cd" />

## Data Flow
<img width="1636" height="341" alt="image" src="https://github.com/user-attachments/assets/81f8fa70-b63b-47ce-82f7-f938b7657c8b" />


## Installation
### 1. Clone the repository
`
git clone https://github.com/kiinitix/streamSync.git
cd streamSync
`

### 2. Start Kafka and the service
`
cd docker
docker compose up --build
`

### 3. Send sample data
`
cd ../tools
pip install confluent-kafka
python producer_examples.py
`

### 4. View reconciliation results
`
docker compose exec kafka bash -c \
"kafka-console-consumer --bootstrap-server localhost:9092 --topic reconcile.events --from-beginning"
`

