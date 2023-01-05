
use log::info;
use cloudevents::{event::Data, Event, EventBuilder, EventBuilderV10};
use serde_json::{from_slice, from_str, json};

pub async fn handle_event(event: Event) -> Result<Event, anyhow::Error> {
    info!("event: {}", event);

    let input = match event.data() {
        Some(Data::Binary(v)) => from_slice(v)?,
        Some(Data::String(v)) => from_str(v)?,
        Some(Data::Json(v)) => v.to_owned(),
        None => json!({ "name": "default" }),
    };

    EventBuilderV10::from(event)
        .source("func://handler")
        .ty("func.example")
        .data("application/json", json!({ "hello": input["name"] }))
        .build()
        .map_err(|err| err.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn post_test() -> Result<(), Box<dyn std::error::Error>> {
        // This is currently failing with
        // Fatal runtime error: assertion failed: thread_info.is_none()
        // [error] execution failed: unreachable, Code: 0x89
        // [error] In instruction: unreachable (0x00) , Bytecode offset: 0x000b8ea9
        // [error] When executing function name: "_start"
        let mut reqevt= Event::default();
        let respevt = handle_event(reqevt).await?;
        let data = respevt.data().unwrap();
        let compare = format!("{}", data);
        assert_eq!(compare.as_str(), "{ \"name\": \"default\" }");
        Ok(())
    }
}