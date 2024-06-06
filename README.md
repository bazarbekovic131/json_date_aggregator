# Actix Web JSON Transformer

This project is a Rust application built using the Actix-web framework. It transforms JSON data by aggregating and formatting it into a specific structure based on the number of days in a month.

## Features

- Parses input JSON containing items with names, types, dates, and amounts.
- Aggregates data by item names and types.
- Distributes amounts into corresponding day fields based on the provided dates.
- Supports different month lengths (28, 29, 30, and 31 days).

## Prerequisites

- Rust programming language: [Install Rust](https://www.rust-lang.org/tools/install)

## Building the Project

To build the project, run the following command:

```sh
cargo build --release
```

This will create an optimized binary in the target/release directory.

## Running the Project Locally

To run the project locally, use the following command:

```sh
cargo run
```

The application will start a server and listen on 127.0.0.1:8080.

## API Endpoint

POST /transform
Transforms the input JSON data and returns the aggregated and formatted data.

Request

Method: POST
URL: http://127.0.0.1:8080/transform
Headers: Content-Type: application/json
Body: JSON array of objects with the following structure:
```json
{"request": [
    {"name": "item1", "type": "kg", "date": "01/01/2024", "amount": 10},
    {"name": "item1", "type": "kg", "date": "02/01/2024", "amount": 20}
]}
```

Response

Content-Type: application/json
Body: JSON array of aggregated objects with the number of days depending on the month:
```json
[
  {
    "name": "item1",
    "type": "kg",
    "day1": 10.0,
    "day2": 0.0,
    "day3": 0.0,
    ...
    "day28": 0.0
  },
  {
    "name": "item2",
    "type": "kg",
    "day1": 0.0,
    "day2": 20.0,
    "day3": 0.0,
    ...
    "day28": 0.0
  }
]
```

# Deployment

To deploy the application as a service on a Linux server using systemd, follow these steps:

Build the project:
```sh
cargo build --release
```

Transfer the binary to the server:

```sh
scp target/release/your_app_name user@your_server_ip:/path/to/your/app
```

Create a systemd service file:
```ini
sudo nano /etc/systemd/system/your_app_name.service
```

Add the following content to the service file:

```ini
[Unit]
Description=Your Actix Web App
After=network.target

[Service]
User=your_user
ExecStart=/path/to/your/app/your_app_name
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Reload systemd and start the service:
```sh
sudo systemctl daemon-reload
sudo systemctl enable your_app_name
sudo systemctl start your_app_name
```

Check the status of the service:
```sh
sudo systemctl status your_app_name
```
You should see the service running.

## Contributing

No contributions are required.

# License

This project is not licensed
