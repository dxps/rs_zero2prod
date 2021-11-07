use std::net::TcpListener;

use rs_zero2prod::{config, startup};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the config and panic if this fails.
    let app_config = config::get_config().expect("Failed to load the app config.");

    let address = format!("127.0.0.1:{}", app_config.http_port);
    let listener = TcpListener::bind(&address).expect(&format!("Failed to listen on {}.", address));

    startup::run(listener)?.await
}
