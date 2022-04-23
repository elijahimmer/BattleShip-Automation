use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "N")]
    North,
    #[serde(rename = "S")]
    South,
    #[serde(rename = "E")]
    East,
    #[serde(rename = "W")]
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rows {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
}

pub fn int2row(i: usize) -> Rows {
    match i {
        0 => Rows::A,
        1 => Rows::B,
        2 => Rows::C,
        3 => Rows::D,
        4 => Rows::E,
        5 => Rows::F,
        6 => Rows::G,
        7 => Rows::H,
        8 => Rows::I,
        _ => panic!("Invalid row number"),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum ShipType {
    None = 0,
    Battleship,
    AttackPlane,
    SpyPlane,
    Fighter,
    Tank,
}
