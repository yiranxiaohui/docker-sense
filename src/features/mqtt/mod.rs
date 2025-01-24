mod recv;

use log::{error, info};
use rumqttc::v5::{Client};
use rumqttc::v5::mqttbytes::QoS;

pub fn init() {
    // info!("初始化MQTT连接！");
    // let mqtt = config::init().get_mqtt();
    // let mut mqtt_options = MqttOptions::new(mqtt.name, mqtt.host, mqtt.port);
    // mqtt_options.set_credentials(mqtt.username, mqtt.password).set_keep_alive(Duration::from_secs(5));
    // let (client, mut connection) = Client::new(mqtt_options, 10);
    // subscribe(client, mqtt.topics);
    // spawn(|| recv(connection));
}

pub fn subscribe(client: Client, topics: Vec<String>) {
    for topic in topics {
        match client.subscribe(topic.as_str(), QoS::AtMostOnce) {
            Ok(_) => {
                info!("{} 订阅成功！", topic);
            }
            Err(_) => {
                error!("===>{}订阅失败！", topic);
            }
        }
    }
}