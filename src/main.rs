pub mod routes;

use crate::routes::*;
use actix_web::{App, HttpServer};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || App::new().service(health_check))
        .bind("127.0.0.1:8000")
        .expect("Binding to 127.0.0.1:8000 failed")
        .run()
        .await
}
