# PingMyIP

A simple Rust library to fetch your public IP address

## Features

- Fetch IP details in JSON format.
- Fetch IP details in plain text format.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pingmyip = "0.0.2"
```

## Usage

Example of using PingMyIP:
```
async fn get_ip_details() {
    let json_details = pingmyip::fetch_ip_json().await.unwrap();
    println!("IP Details in JSON: {}", json_details);

    let text_details = pingmyip::fetch_ip_text().await.unwrap();
    println!("IP Details in Text: {}", text_details);
}
```