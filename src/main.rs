use paho_mqtt as mqtt;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // MQTT broker configuration
    let broker_host = "your_mqtt_broker.com"; // e.g., "broker.emqx.io"
    let broker_port = 443; // 443 for secure WebSocket | 1883 for standard MQTT | 8883 for secure MQTT
    let username = "your_username";
    let password = "your_password";
    let client_id = "iot_device_rust_001";
    let proto = "wss"; // Use "ws" for non-secure WebSocket | "tcp" for standard MQTT | "ssl" for secure MQTT
    let qos = 1; // Quality of Service level for MQTT
    let path = "/mqtt"; // MQTT over WebSocket path (use your broker's specific path if needed)
    let ssl = true; // Set to true if using secure connection

    // Create MQTT client
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(format!("{}://{}:{}{}", proto, broker_host, broker_port, path))
        .client_id(client_id)
        .finalize();

    let mut client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating MQTT client: {}", err);
        std::process::exit(1);
    });

    // Set up message callback for incoming messages
    let rx = client.get_stream(25);

    // Connection options with authentication
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .user_name(username)
        .password(password).ssl_options(
            mqtt::SslOptionsBuilder::new()
                .enable_server_cert_auth(ssl)
                .finalize(),
        )
        .finalize();

    // Connect to broker
    match client.connect(conn_opts).await {
        Ok(_) => println!("Connected to MQTT broker: {}", broker_host),
        Err(err) => {
            println!("Error connecting to MQTT broker: {}", err);
            std::process::exit(1);
        }
    }

    // Subscribe to IoT topics
    let topics = vec![
        "iot/sensors/temperature",
        "iot/sensors/humidity",
        "iot/devices/status",
    ];

    for topic in &topics {
        match client.subscribe(*topic, qos).await {
            Ok(_) => println!("Subscribed to topic: {}", topic),
            Err(err) => println!("Error subscribing to {}: {}", topic, err),
        }
    }

    // Spawn task to handle incoming messages
    tokio::spawn(async move {
        while let Ok(msg_opt) = rx.recv().await {
            if let Some(msg) = msg_opt {
                let topic = msg.topic();
                let payload = msg.payload_str();
                println!("Received message on {}: {}", topic, payload);

                // Process different types of IoT data
                match topic {
                    "iot/sensors/temperature" => {
                        if let Ok(temp) = payload.parse::<f32>()
                            && temp > 30.0
                        {
                            println!("⚠️  High temperature alert: {}°C", temp);
                        }
                    }
                    "iot/sensors/humidity" => {
                        if let Ok(humidity) = payload.parse::<f32>()
                            && humidity < 30.0
                        {
                            println!("💧 Low humidity alert: {}%", humidity);
                        }
                    }
                    "iot/devices/status" => {
                        println!("📱 Device status update: {}", payload);
                    }
                    _ => {}
                }
            }
        }
    });

    // Simulate IoT sensor data publishing
    let mut counter = 0;
    loop {
        // Simulate temperature sensor data
        let temperature = 20.0 + (counter as f32 * 0.5) % 20.0;
        let temp_msg = mqtt::Message::new("iot/sensors/temperature", temperature.to_string(), 1);

        if let Err(err) = client.publish(temp_msg).await {
            println!("Error publishing temperature: {}", err);
        } else {
            println!("📊 Published temperature: {:.1}°C", temperature);
        }

        // Simulate humidity sensor data
        let humidity = 40.0 + (counter as f32 * 0.3) % 40.0;
        let humidity_msg = mqtt::Message::new("iot/sensors/humidity", humidity.to_string(), 1);

        if let Err(err) = client.publish(humidity_msg).await {
            println!("Error publishing humidity: {}", err);
        } else {
            println!("💧 Published humidity: {:.1}%", humidity);
        }

        // Simulate device status
        let status = if counter % 10 == 0 {
            "online"
        } else {
            "active"
        };
        let status_msg = mqtt::Message::new(
            "iot/devices/status",
            format!(
                "{{\"device_id\": \"{}\", \"status\": \"{}\", \"uptime\": {}}}",
                client_id,
                status,
                counter * 5
            ),
            1,
        );

        if let Err(err) = client.publish(status_msg).await {
            println!("Error publishing status: {}", err);
        } else {
            println!("📱 Published device status: {}", status);
        }

        counter += 1;

        // Wait 5 seconds before next publication
        time::sleep(Duration::from_secs(5)).await;
    }
}
