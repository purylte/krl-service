use serde::Serialize;
use strum_macros::EnumString;

use crate::station::Station;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, EnumString)]
pub enum TrainLine {
    B,
    C,
    R,
    TP,
    T,
}

impl TrainLine {
    pub fn name(&self) -> &str {
        match *self {
            TrainLine::B => "Lin Bogor",
            TrainLine::C => "Lin Lingkar Cikarang",
            TrainLine::R => "Lin Rangkasbitung",
            TrainLine::TP => "Lin Tanjung Priok",
            TrainLine::T => "Lin Tangerang",
        }
    }

    pub fn neighbour(&self) -> Vec<NeighbouringLine> {
        match *self {
            TrainLine::B => vec![
                NeighbouringLine {
                    line: TrainLine::TP,
                    transit_station: Station::JAKK,
                },
                NeighbouringLine {
                    line: TrainLine::C,
                    transit_station: Station::MRI,
                },
            ],
            TrainLine::C => vec![
                NeighbouringLine {
                    line: TrainLine::TP,
                    transit_station: Station::KPB,
                },
                NeighbouringLine {
                    line: TrainLine::B,
                    transit_station: Station::MRI,
                },
                NeighbouringLine {
                    line: TrainLine::T,
                    transit_station: Station::DU,
                },
                NeighbouringLine {
                    line: TrainLine::R,
                    transit_station: Station::THB,
                },
            ],
            TrainLine::R => vec![NeighbouringLine {
                line: TrainLine::C,
                transit_station: Station::THB,
            }],
            TrainLine::TP => vec![
                NeighbouringLine {
                    line: TrainLine::B,
                    transit_station: Station::JAKK,
                },
                NeighbouringLine {
                    line: TrainLine::C,
                    transit_station: Station::KPB,
                },
            ],
            TrainLine::T => vec![NeighbouringLine {
                line: TrainLine::C,
                transit_station: Station::DU,
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NeighbouringLine {
    pub line: TrainLine,
    pub transit_station: Station,
}
