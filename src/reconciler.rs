use crate::state_store::StateStore;
use serde_json::{json, Value};

pub struct Reconciler {
    store: StateStore,
}

impl Reconciler {
    pub fn new() -> Self {
        Self { store: StateStore::new() }
    }

    pub fn ingest(&mut self, source: &str, event: Value) {
        if let Some(id) = event.get("entity_id").and_then(|v| v.as_str()) {
            self.store.insert(source, id.to_string(), event);
        } else {
        }
    }

    pub fn try_reconcile_for_message(&self, source: &str, event: &Value) -> Option<Value> {
        let key = event.get("entity_id")?.as_str()?;
        let other_source = if source == "svcA.changes" { "svcB.changes" } else { "svcA.changes" };

        let mine = self.store.get(source, key);
        let other = self.store.get(other_source, key);

        match (mine, other) {
            (Some(mv), Some(ov)) => {
                let field = "status";
                let my_field = mv.get(field);
                let other_field = ov.get(field);
                if my_field == other_field {
                    Some(json!({
                        "entity_id": key,
                        "type": "match",
                        "field": field,
                        "value": my_field,
                        "sources": [source, other_source]
                    }))
                } else {
                    Some(json!({
                        "entity_id": key,
                        "type": "drift",
                        "field": field,
                        "svcA": if source.starts_with("svcA") { my_field } else { other_field },
                        "svcB": if source.starts_with("svcB") { my_field } else { other_field }
                    }))
                }
            }
            _ => None,
        }
    }
}
