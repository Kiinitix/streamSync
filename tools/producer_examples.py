from confluent_kafka import Producer
import json, time

p = Producer({'bootstrap.servers': 'localhost:29092'})
svcA = [
  {"entity_id":"order-1001","status":"created"},
  {"entity_id":"order-1002","status":"processing"},
  {"entity_id":"order-1003","status":"shipped"}
]
svcB = [
  {"entity_id":"order-1001","status":"created"},
  {"entity_id":"order-1002","status":"completed"},
  {"entity_id":"order-1004","status":"created"}
]

for msg in svcA:
    p.produce('svcA.changes', key=msg['entity_id'], value=json.dumps(msg))
    p.flush()

time.sleep(1)

for msg in svcB:
    p.produce('svcB.changes', key=msg['entity_id'], value=json.dumps(msg))
    p.flush()
