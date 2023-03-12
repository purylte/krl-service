use std::net::ToSocketAddrs;

use actix_web::{App, HttpServer};
use route::{
    distance, get_fastest_route, get_transit_route, station_list, station_schedule, train_fare,
    train_schedule, line_list,
};

mod error;
mod fetch;
mod line;
mod model;
mod pathfinder;
mod route;
mod station;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let ip_port = &args[1].to_socket_addrs()?.next().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(station_schedule)
            .service(station_list)
            .service(train_schedule)
            .service(train_fare)
            .service(distance)
            .service(get_fastest_route)
            .service(get_transit_route)
            .service(line_list)
    })
    .bind(ip_port)?
    .run()
    .await
}
