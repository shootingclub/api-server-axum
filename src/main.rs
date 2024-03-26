mod init;
mod entity;
mod controller;
mod comm;
mod service;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use axum::Server;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = init::init_router();
    let addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


