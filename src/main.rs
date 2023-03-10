use actix_web::{App, HttpServer};
use route::{
    distance, get_fastest_route, get_transit_route, station_list, station_schedule,
    train_fare, train_schedule,
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
    HttpServer::new(|| {
        App::new()
            .service(station_schedule)
            .service(station_list)
            .service(train_schedule)
            .service(train_fare)
            .service(distance)
            .service(get_fastest_route)
            .service(get_transit_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
