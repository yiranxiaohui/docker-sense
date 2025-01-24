use log::info;
use rumqttc::v5::{Connection, Event, Incoming};
use crate::model::Sql;

pub fn recv(mut connection: Connection) {
    for (_i, notification) in connection.iter().enumerate() {
        if let Ok(event) = notification {
            if let Event::Incoming(input) = event {
                if let Incoming::Publish(publish) = input {
                    let topic = String::from_utf8(publish.topic.to_vec()).unwrap();
                    let payload = String::from_utf8(publish.payload.to_vec()).unwrap();
                    if topic.eq("/sql/send") {
                        if let Ok(sql) = serde_json::from_str::<Sql>(payload.as_str()) {
                            info!("sql = {:?}", sql);
                        }
                    }
                }
            }
        }
    }
}