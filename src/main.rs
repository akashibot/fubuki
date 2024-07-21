#![feature(slice_pattern)]
extern crate core;

use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sysinfo::System;

mod middlewares;
mod routes;
mod utils;

#[get("/health")]
async fn health() -> impl Responder {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    sys.refresh_memory();

    // Get the current process
    let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
    let process = sys.process(pid).expect("Failed to get process info");

    let memory_usage = process.memory() / 1024 / 1024;
    let cpu_usage = process.cpu_usage();

    HttpResponse::Ok()
        .insert_header(CacheControl(vec![CacheDirective::NoCache]))
        .json(json!({
            "alive": true,
            "memory": format!("{} MB", memory_usage),
            "cpu": format!("{:.1}%", cpu_usage),
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // .wrap(middleware::DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
            .service(health)
            .service(
                web::scope("/api")
                    .wrap(middlewares::image::ImageParser)
                    .service(routes::invert::invert)
                    .service(routes::speech::speech)
                    .service(routes::caption::caption)
                    .service(routes::opacity::opacity)
                    .service(routes::convert::convert)
                    .service(routes::circle::circle),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
