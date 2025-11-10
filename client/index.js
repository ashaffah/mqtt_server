(async () => {
  const broker_host = "your-mqtt-broker.com";
  const broker_port = 443;
  const username = "your_username";
  const password = "your_password";
  const client_id = "iot_device_rust_002";
  const proto = "wss";
  const qos = 1;
  const path = "/mqtt";

  const topics = ["iot/sensors/temperature", "iot/sensors/humidity", "iot/devices/status"];

  const client = mqtt.connect("" + proto + "://" + broker_host + ":" + broker_port + path, {
    clientId: client_id,
    username: username,
    password: password,
    qos: qos,
    rejectUnauthorized: false,
    protocol: proto === "wss" ? "wss" : proto === "ws" ? "ws" : proto === "ssl" ? "mqtts" : "mqtt",
  });

  client.on("connect", () => {
    console.log("Connected to MQTT broker");

    // Subscribe to all topics
    topics.forEach((topic) => {
      client.subscribe(topic, { qos: qos }, (err) => {
        if (err) {
          console.error(`Error subscribing to ${topic}:`, err);
        } else {
          console.log(`Subscribed to topic: ${topic}`);
        }
      });
    });
  });

  client.on("error", (err) => {
    console.error("MQTT error:", err);
  });

  client.on("message", (topic, message) => {
    const msg = message.toString();
    console.log(`Received message on ${topic}: ${msg}`);

    const messagesDiv = document.getElementById("messages");
    const msgElement = document.createElement("div");
    msgElement.style.padding = "10px";
    msgElement.style.margin = "5px";
    msgElement.style.border = "1px solid #ccc";
    msgElement.style.borderRadius = "5px";

    const timestamp = new Date().toLocaleTimeString();
    msgElement.innerHTML = `<strong>${timestamp}</strong> - <em>${topic}</em>: ${msg}`;

    messagesDiv.prepend(msgElement);
  });
})();
