pub mod for_async {

    use influxdb::Client;
    use std::cell::RefCell;

    thread_local! {
        static INFLUX_CLIENT : RefCell<influxdb::Client> = RefCell::new( init_influx_cli() );
    }

    pub fn get_influx_cli() -> influxdb::Client {
        INFLUX_CLIENT.with(|rc| rc.borrow().clone())
    }

    fn init_influx_cli() -> influxdb::Client {
        let host = dotenv::var("INFLUX_URL").unwrap_or_else(|_| {
            println! {"INFLUX_URL not found in .evn file, using default: http://localhost:8086"};
            "http://localhost:8086".to_string()
        });

        let db = dotenv::var("INFLUX_DB").expect("INFLUX_DB not defined in .env file");
        let user = dotenv::var("INFLUX_USER").expect("INFLUX_USER not defined in .env file");
        let pass =
            dotenv::var("INFLUX_PASSWORD").expect("INFLUX_PASSWORD not defined in .env file");

        Client::new(host, db).with_auth(user, pass)
    }

    use mqtt::{AsyncClient, Message};
    use paho_mqtt as mqtt;
    use std::{process, time::Duration};

    pub async fn get_mqtt_cli_and_stream(
    ) -> (mqtt::AsyncClient, mqtt::AsyncReceiver<Option<Message>>) {
        let host = dotenv::var("MQTT_BROKER").unwrap_or_else(|_| {
            println! {"MQTT_BROKER not found in .evn file, using default: tcp://localhost:1883"};
            "tcp://localhost:1883".to_string()
        });

        // Create the client. Use an ID for a persistent session.
        // A real system should try harder to use a unique ID.
        let mqtt_options = mqtt::CreateOptionsBuilder::new()
            .server_uri(host)
            .client_id("mmRTLS_async_subscribe")
            .finalize();

        // Create the client connection
        let mut client = AsyncClient::new(mqtt_options).unwrap_or_else(|e| {
            println!("Error creating the client: {:?}", e);
            process::exit(1);
        });

        // Get message stream before connecting.
        let stream = client.get_stream(25);

        // Define the set of options for the connection
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        client.connect(conn_opts).await.unwrap_or_else(|e| {
            println!("Error connecting to the broker: {:?}", e);
            process::exit(1);
        });

        return (client, stream);
    }

    pub async fn mqtt_cli_reconnect(client: &mqtt::AsyncClient) {
        println!("Lost connection. Attempting reconnect.");
        while let Err(err) = client.reconnect().await {
            println!("Error reconnecting: {}", err);
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    }

    pub async fn mqtt_subscribe(client: &mqtt::AsyncClient, topic: &str) {
        client
            .subscribe(topic, 1)
            .await
            .expect("Unable to subscribe");
    }
}

pub mod for_sync {
    use paho_mqtt as mqtt;
    use std::{process, time::Duration};

    pub fn get_mqtt_cli() -> mqtt::Client {
        let host = dotenv::var("MQTT_BROKER").unwrap_or_else(|_| {
            println! {"MQTT_BROKER not found in .evn file, using default: tcp://localhost:1883"};
            "tcp://localhost:1883".to_string()
        });

        let mut cli = mqtt::Client::new(host).unwrap_or_else(|e| {
            println!("Error creating the client: {:?}", e);
            process::exit(1);
        });

        // Use 5sec timeouts for sync calls.
        cli.set_timeout(Duration::from_secs(5));

        // Connect and wait for it to complete or fail
        if let Err(e) = cli.connect(None) {
            println!("Unable to connect: {:?}", e);
            process::exit(1);
        }
        return cli;
    }

    pub fn mqtt_pub(
        client: &mqtt::Client,
        topic: &str,
        payload: &str,
    ) -> Result<(), paho_mqtt::Error> {
        let msg = mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload)
            .qos(1)
            .finalize();

        client.publish(msg)
    }
}
