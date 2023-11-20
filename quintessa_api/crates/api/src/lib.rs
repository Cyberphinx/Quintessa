use app_state::AppState;
use axum_server::tls_rustls::RustlsConfig;
use router::create_router;
use std::{env, net::SocketAddr};

pub mod app_state;
pub mod controllers;
mod middleware;
pub mod migrations;
mod router;
pub mod utilities;

pub async fn run(app_state: AppState) {
    let app = create_router(app_state);
    let address: SocketAddr;
    if env::var("ENVIRONMENT").unwrap().eq("production") {
        // FLYIO
        address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], 8080));

        // HEROKU
        // read the port from env or use the port default port(8080)
        // let port = std::env::var("PORT").unwrap_or(String::from("8080"));
        // convert the port to a socket address
        // address = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

        println!("Running on address: {}", &address);
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        address = SocketAddr::from(([0, 0, 0, 0], 5002));
        println!("Listening on address: {}", &address);

        let config = RustlsConfig::from_pem_file("./cert/cert.pem", "./cert/key.pem")
            .await
            .unwrap();

        axum_server::bind_rustls(address, config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
