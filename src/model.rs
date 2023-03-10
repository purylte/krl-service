use std::str::FromStr;

use chrono::NaiveTime;
use serde_derive::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppErrorType},
    station::Station,
};

#[derive(Deserialize, Debug)]
pub struct StationScheduleDTO {
    train_id: String,
    route_name: String,
    time_est: String,
}

#[derive(Deserialize, Debug)]
pub struct TrainScheduleDTO {
    train_id: String,
    station_id: String,
    time_est: String,
}

#[derive(Deserialize, Debug)]
pub struct RouteInfoDTO {
    fare: u16,
    distance: String,
}

#[derive(Serialize)]
pub struct StationSchedule {
    pub train_id: String,
    pub route_name: String,
    pub time_est: NaiveTime,
}

impl StationSchedule {
    pub fn from_dto(value: StationScheduleDTO) -> Result<Self, AppError> {
        let time_est = to_naive_time_hm(value.time_est)?;

        return Ok(Self {
            train_id: value.train_id,
            route_name: value.route_name,
            time_est,
        });
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct TrainSchedule {
    pub train_id: String,
    pub station: Station,
    pub time_est: NaiveTime,
}

impl TrainSchedule {
    pub fn from_dto(value: TrainScheduleDTO) -> Result<Self, AppError> {
        let train_id = value.train_id;
        let station = Station::from_str(&value.station_id)?;
        let time_est = to_naive_time_hm(value.time_est)?;

        return Ok(Self {
            train_id,
            station,
            time_est,
        });
    }
}

#[derive(Serialize)]
pub struct Fare {
    pub fare: u16,
}

impl From<RouteInfoDTO> for Fare {
    fn from(value: RouteInfoDTO) -> Self {
        Self { fare: value.fare }
    }
}

#[derive(Serialize)]
pub struct Distance {
    pub distance: f32,
}

impl From<RouteInfoDTO> for Distance {
    fn from(value: RouteInfoDTO) -> Self {
        Self {
            distance: value.distance.parse().unwrap_or(-1.),
        }
    }
}

pub fn to_naive_time_hm(time: String) -> Result<NaiveTime, AppError> {
    let err = AppError {
        message: Some("Invalid time format".into()),
        cause: None,
        error_type: AppErrorType::InvalidTimeFormat,
    };

    let time = time.split(":").collect::<Vec<&str>>();
    let Ok(hour) = time[0].parse::<u32>() else {
        return Err(err);
    };
    let Ok(min) = time[1].parse::<u32>() else {
        return Err(err);
    };
    NaiveTime::from_hms_opt(hour, min, 0).ok_or(err)
}

pub fn to_naive_time_hms(time: String) -> Result<NaiveTime, AppError> {
    let err = AppError {
        message: Some("Invalid time format".into()),
        cause: None,
        error_type: AppErrorType::InvalidTimeFormat,
    };

    let time = time.split(":").collect::<Vec<&str>>();
    let Ok(hour) = time[0].parse::<u32>() else {
        return Err(err);
    };
    let Ok(min) = time[1].parse::<u32>() else {
        return Err(err);
    };
    let Ok(sec) = time[2].parse::<u32>() else {
        return Err(err);
    };
    NaiveTime::from_hms_opt(hour, min, sec).ok_or(err)
}
