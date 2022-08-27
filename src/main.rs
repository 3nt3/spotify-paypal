#[macro_use]
extern crate tokio;

use serde::Deserialize;
use std::{collections::HashMap, fs};

use toml;

#[derive(Debug, Deserialize)]
struct LoginResponse {
    access_token: String,
    home_server: String,
    user_id: String,
}

#[derive(Debug, Deserialize)]
struct MsgResponse {
    event_id: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    user: String,
    password: String,
    room_id: String,
    homeserver: String,
}

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("config.toml").unwrap();
    let config = toml::from_str::<Config>(&config_str).unwrap();
    // https://www.paypal.com/cgi-bin/webscr?cmd=_xclick&business=YOUR_PAYPAL_EMAIL&amount=10&currency_code=USD&item_name=test

    // curl -XPOST -d '{"type":"m.login.password", "user":"example", "password":"wordpass"}' "https://localhost:8448/_matrix/client/r0/login"
    let homeserver = &config.homeserver;
    let user = &config.user;
    let password = &config.password;

    let client = reqwest::Client::new();

    let mut login_body = HashMap::new();
    login_body.insert("type", "m.login.password");
    login_body.insert("user", user);
    login_body.insert("password", password);

    let login_res = client
        .post(format!("{homeserver}/_matrix/client/r0/login"))
        .json(&login_body)
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap();

    let access_token = &login_res.access_token;
    dbg!(&login_res);

    // curl -XPOST -d '{"msgtype":"m.text", "body":"hello"}' "https://localhost:8448/_matrix/client/r0/rooms/%21asfLdzLnOdGRkdPZWu:localhost/send/m.room.message?access_token=YOUR_ACCESS_TOKEN"

    // {
    //     "event_id": "YUwRidLecu"
    // }
    // let room_id = "#botspam:3nt3.de";
    // let room_id = "!cqMYEoMzxgZUkxoUYU:3nt3.de";
    let room_id = config.room_id;

    let paypal_email = "niels-schlegel@gmx.de";
    let amount = &(15.0 / 6.0).to_string();
    let paypal_item_name = "Spotify (wir sind wie eine Familie)";

    let paypal_url = format!("https://www.paypal.com/cgi-bin/webscr?cmd=_xclick&business={paypal_email}&amount={amount}&currency_code=EUR&item_name={}", urlencoding::encode(paypal_item_name));

    let mut msg_body = HashMap::new();
    let body = format!("GEBT MIR GELD {paypal_url}");
    msg_body.insert("msgtype", "m.text");
    msg_body.insert("body", body.as_str());

    let msg_res = client
        .post(format!(
            "{homeserver}/_matrix/client/r0/rooms/{room_id}/send/m.room.message"
        ))
        .query(&[("access_token", access_token)])
        .json(&msg_body)
        .send()
        .await
        .unwrap()
        .text()
        .await;

    dbg!(msg_res);
}
