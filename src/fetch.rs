use chrono::NaiveTime;
use serde_derive::Deserialize;

use crate::error::AppError;
use crate::error::AppErrorType;
use crate::model::Distance;
use crate::model::Fare;
use crate::model::RouteInfoDTO;
use crate::model::StationSchedule;
use crate::model::StationScheduleDTO;
use crate::model::TrainSchedule;
use crate::model::TrainScheduleDTO;
use crate::station::Station;

#[derive(Deserialize, Debug)]
pub struct APIResponse<T> {
    data: Vec<T>,
}

pub async fn fetch_station_schedule(
    station: Station,
    time_from: NaiveTime,
    time_to: NaiveTime,
) -> Result<Vec<StationSchedule>, AppError> {
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api-partner.krl.co.id/krlweb/v1/schedule?stationid={}&timefrom={}&timeto={}",
        station.id(),
        time_from.format("%H:%M"),
        time_to.format("%H:%M")
    );

    let res = client.get(url).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let dtos = res.json::<APIResponse<StationScheduleDTO>>().await?.data;
            let schedules_result: Result<Vec<StationSchedule>, AppError> = dtos
                .into_iter()
                .map(|dto| StationSchedule::from_dto(dto))
                .collect();
            Ok(schedules_result?)
        }
        _ => Err(AppError {
            message: Some("Failed to fetch station schedule".into()),
            cause: None,
            error_type: AppErrorType::NotFoundError,
        }),
    }
}

pub async fn fetch_train_schedule(train_id: &str) -> Result<Vec<TrainSchedule>, AppError> {
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api-partner.krl.co.id/krlweb/v1/schedule-train?trainid={}",
        train_id
    );

    let res = client.get(url).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => {
            let dtos = res.json::<APIResponse<TrainScheduleDTO>>().await?.data;
            let schedules_result: Result<Vec<TrainSchedule>, AppError> = dtos
                .into_iter()
                .map(|dto| TrainSchedule::from_dto(dto))
                .collect();
            Ok(schedules_result?)
        }

        _ => Err(AppError {
            message: Some("Failed to fetch train schedule".into()),
            cause: None,
            error_type: AppErrorType::ReqwestError,
        }),
    }
}

pub async fn fetch_fare(station_from: Station, station_to: Station) -> Result<Fare, AppError> {
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api-partner.krl.co.id/krlweb/v1/fare?stationfrom={}&stationto={}",
        station_from.id(),
        station_to.id()
    );

    let res = client.get(url).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let route_info = res.json::<APIResponse<RouteInfoDTO>>().await?.data;
            Ok(route_info.into_iter().next().unwrap().into())
        }
        _ => Err(AppError {
            message: Some("Failed to fetch fare".into()),
            cause: None,
            error_type: AppErrorType::NotFoundError,
        }),
    }
}

pub async fn fetch_distance(
    station_from: Station,
    station_to: Station,
) -> Result<Distance, AppError> {
    let client = reqwest::Client::builder().build()?;

    let url = format!(
        "https://api-partner.krl.co.id/krlweb/v1/fare?stationfrom={}&stationto={}",
        station_from.id(),
        station_to.id()
    );

    let res = client.get(url).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            let route_info = res.json::<APIResponse<RouteInfoDTO>>().await?.data;
            Ok(route_info.into_iter().next().unwrap().into())
        }
        _ => Err(AppError {
            message: Some("Failed to fetch distance".into()),
            cause: None,
            error_type: AppErrorType::NotFoundError,
        }),
    }
}
