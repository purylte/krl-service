use std::collections::HashSet;

use chrono::{Duration, NaiveDateTime, NaiveTime};

use crate::{
    error::{AppError, AppErrorType},
    fetch::{fetch_station_schedule, fetch_train_schedule},
    line::{NeighbouringLine, TrainLine},
    model::TrainSchedule,
    station::Station,
};

fn dfs(
    from: NeighbouringLine,
    to: TrainLine,
    path: &mut Vec<NeighbouringLine>,
    mut paths: Vec<Vec<NeighbouringLine>>,
    on_path: &mut HashSet<TrainLine>,
) -> Vec<Vec<NeighbouringLine>> {
    path.push(from.clone());
    on_path.insert(from.line.clone());
    if from.line == to {
        paths.push(path.clone());
    } else {
        for neighbour in from.line.neighbour() {
            if !on_path.contains(&neighbour.line) {
                paths = dfs(neighbour, to, path, paths, on_path);
            }
        }
    }

    path.pop();
    on_path.remove(&from.line);
    paths
}

fn generate_all_transit_paths(from: Station, to: Station) -> Vec<Vec<Station>> {
    let mut station_pathss = vec![];
    for from_line in from.line() {
        for to_line in to.line() {
            let station_paths = dfs(
                NeighbouringLine {
                    line: from_line.clone(),
                    transit_station: from,
                },
                to_line.clone(),
                &mut vec![],
                vec![],
                &mut HashSet::new(),
            )
            .into_iter()
            .filter(|station_path| station_path.last().unwrap().transit_station != to)
            .filter(|station_path| {
                let mut iter = station_path.iter();
                let first = iter.next();
                let second = iter.next();
                if let (Some(first), Some(second)) = (first, second) {
                    first.transit_station != second.transit_station
                } else {
                    true
                }
            })
            .map(|station_path| {
                let mut stations = vec![];
                for path in station_path {
                    stations.push(path.transit_station);
                }
                stations.push(to);
                stations
            });

            station_pathss.extend(station_paths);
        }
    }
    station_pathss
}

pub fn generate_all_transit_routes(
    from: Station,
    to: Station,
) -> Result<Vec<Vec<Station>>, AppError> {
    if from == to {
        Err(AppError {
            message: Some("Station from and station cannot be the same".into()),
            cause: None,
            error_type: AppErrorType::InvalidRequestParameter,
        })
    } else {
        Ok(generate_all_transit_paths(from, to))
    }
}

async fn get_first_train_schedule_to_station_same_line(
    from: Station,
    to: Station,
    time_start: NaiveTime,
) -> Result<Vec<TrainSchedule>, AppError> {
    let first = fetch_station_schedule(from, time_start, NaiveDateTime::MAX.time()).await?;
    for train in first {
        let train_schedules = fetch_train_schedule(&train.train_id).await?;
        let mut start_index = None;
        let mut stop_index = None;
        for (i, schedule) in train_schedules.iter().enumerate() {
            if schedule.station == from {
                start_index = Some(i);
            }
            if schedule.station == to {
                stop_index = Some(i + 1);
                break;
            }
        }

        if let (Some(start_index), Some(stop_index)) = (start_index, stop_index) {
            if start_index < stop_index {
                return Ok(train_schedules[start_index..stop_index].to_vec());
            }
        }
    }
    Err(AppError {
        message: None,
        cause: Some(format!(
            "No train found from {}({}) to {}({})",
            from.name(),
            from.id(),
            to.name(),
            to.id()
        )),
        error_type: crate::error::AppErrorType::NotFoundError,
    })
}

async fn concat_train_schedule_path_from_station_path(
    path: Vec<Station>,
    time_start: NaiveTime,
    transit_duration: Duration,
) -> Result<Vec<TrainSchedule>, AppError> {
    let mut train_schedule_path = Vec::new();
    let mut from_time = time_start;

    // println!("path: {:#?}", path);
    for (i, station) in path.iter().enumerate() {
        if i == path.len() - 1 {
            break;
        }
        let next_station = path[i + 1].clone();
        let train_schedule =
            get_first_train_schedule_to_station_same_line(station.clone(), next_station, from_time)
                .await?;
        // println!("train_schedule: {:#?}", train_schedule);
        from_time = train_schedule.last().unwrap().time_est + transit_duration;
        train_schedule_path.extend(train_schedule);
    }
    Ok(train_schedule_path)
}

async fn generate_and_concat_train_schedule_path(
    station_from: Station,
    station_to: Station,
    time_start: NaiveTime,
    transit_duration: Duration,
) -> Result<Vec<Vec<TrainSchedule>>, AppError> {
    let paths = generate_all_transit_paths(station_from, station_to);
    let mut train_schedule_path = Vec::new();
    for path in paths.into_iter() {
        let train_schedules =
            concat_train_schedule_path_from_station_path(path, time_start, transit_duration).await?;
        train_schedule_path.push(train_schedules);
    }
    Ok(train_schedule_path)
}

pub async fn choose_fastest_path(
    station_from: Station,
    station_to: Station,
    time_start: NaiveTime,
    transit_duration: Duration,
) -> Result<Vec<TrainSchedule>, AppError> {
    let paths =
        generate_and_concat_train_schedule_path(station_from, station_to, time_start, transit_duration).await?;
    let mut fastest_path = None;
    let mut fastest_time = chrono::Duration::max_value();
    for path in paths.into_iter() {
        if let Some(last_train_schedule) = path.last() {
            if let Some(first_train_schedule) = path.first() {
                let time_diff = last_train_schedule.time_est - first_train_schedule.time_est;
                if time_diff < fastest_time {
                    fastest_time = time_diff;
                    fastest_path = Some(path);
                }
            }
        }
    }
    let Some(fastest_path) = fastest_path else {
        return Err(AppError {
            message: None,
            cause: Some(format!(
                "No path found from {}({}) to {}({})",
                station_from.name(),
                station_from.id(),
                station_to.name(),
                station_to.id()
            )),
            error_type: crate::error::AppErrorType::NotFoundError,
        });
    };
    Ok(fastest_path)
}
