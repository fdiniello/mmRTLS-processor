mod handler;

use tokio;

use common::helper::for_async::{get_mqtt_cli_and_stream, mqtt_cli_reconnect, mqtt_subscribe};
use futures::stream::StreamExt;

#[tokio::main]
async fn main() {
    let (mqtt_cli, mut stream) = get_mqtt_cli_and_stream().await;

    let topic = "device/+/report";
    println!("Subscribing to topic: {:?}", topic);

    mqtt_subscribe(&mqtt_cli, topic).await;

    while let Some(msg) = stream.next().await {
        if let Some(msg) = msg {
            // split the topic first
            let topic: Vec<&str> = msg.topic().splitn(3, '/').collect();

            match topic[0] {
                "device" => match topic[2] {
                    "report" => handler::device::report(topic[1], &msg.payload_str()).await,
                    _ => println!("Unhandled topic for device: {}", topic[2]),
                },
                _ => println!("Unhandled topic: {}", msg.topic()),
            }
        } else {
            // A "None" means we were disconnected. Try to reconnect...
            mqtt_cli_reconnect(&mqtt_cli).await;
            mqtt_subscribe(&mqtt_cli, topic).await;
        }
    }
}
