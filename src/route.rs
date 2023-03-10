use std::str::FromStr;

use actix_web::{get, web, HttpResponse, Responder, Result};
use chrono::NaiveTime;
use serde_derive::Deserialize;

use crate::{
    error::AppError,
    fetch::fetch_distance,
    fetch::fetch_fare,
    fetch::fetch_station_schedule,
    fetch::fetch_train_schedule,
    pathfinder::{choose_fastest_path, generate_all_transit_routes},
    station::Station,
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StationScheduleRequestParam {
    station: String,
    time_from: NaiveTime,
    time_to: NaiveTime,
}

#[get("/station-schedule")]
async fn station_schedule(
    req: web::Query<StationScheduleRequestParam>,
) -> Result<HttpResponse, AppError> {
    let station = Station::from_str(&req.station)?;
    let station_schedule = fetch_station_schedule(station, req.time_from, req.time_to).await?;
    Ok(HttpResponse::Ok().json(station_schedule))
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct TrainScheduleRequestParam {
    train_id: String,
}

#[get("/train-schedule")]
async fn train_schedule(
    req: web::Query<TrainScheduleRequestParam>,
) -> Result<HttpResponse, AppError> {
    let train_schedule = fetch_train_schedule(&req.train_id).await?;
    Ok(HttpResponse::Ok().json(train_schedule))
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RouteInfoParam {
    station_from: String,
    station_to: String,
}

#[get("/fare")]
async fn train_fare(req: web::Query<RouteInfoParam>) -> Result<HttpResponse, AppError> {
    let station_from = Station::from_str(&req.station_from)?;
    let station_to = Station::from_str(&req.station_to)?;
    let fare = fetch_fare(station_from, station_to).await?;
    Ok(HttpResponse::Ok().json(fare))
}

#[get("/distance")]
async fn distance(req: web::Query<RouteInfoParam>) -> Result<HttpResponse, AppError> {
    let station_from = Station::from_str(&req.station_from)?;
    let station_to = Station::from_str(&req.station_to)?;
    let distance = fetch_distance(station_from, station_to).await?;
    Ok(HttpResponse::Ok().json(distance))
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct PathfindParam {
    station_from: String,
    station_to: String,
    time_from: NaiveTime,
}

#[get("/get-fastest-route")]
async fn get_fastest_route(req: web::Query<PathfindParam>) -> Result<HttpResponse, AppError> {
    let station_from = Station::from_str(&req.station_from)?;
    let station_to = Station::from_str(&req.station_to)?;
    let path = choose_fastest_path(station_from, station_to, req.time_from).await?;
    Ok(HttpResponse::Ok().json(path))
}

#[get("/get-all-transit-route")]
async fn get_transit_route(req: web::Query<PathfindParam>) -> Result<HttpResponse, AppError> {
    let station_from = Station::from_str(&req.station_from)?;
    let station_to = Station::from_str(&req.station_to)?;
    let path = generate_all_transit_routes(station_from, station_to)?;
    Ok(HttpResponse::Ok().json(path))
}

#[get("/station-list")]
async fn station_list() -> Result<impl Responder> {
    Ok(web::Json(Station::json()))
}