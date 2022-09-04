extern crate core;

mod controller;
mod service;
mod model;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controller::car_controller::calculate_diesel_usage_for_distance)
            .service(controller::car_controller::probability_of_unit_injector_fail)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}