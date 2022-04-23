use super::enums::Direction;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    #[serde(rename = "BattleShip")]
    pub battle_ship: Ship,
    #[serde(rename = "AttackPlane")]
    pub attack_plane: Ship,
    #[serde(rename = "SpyPlane")]
    pub spy_plane: Ship,
    #[serde(rename = "Fighter")]
    pub fighter: Ship,
    #[serde(rename = "Tank")]
    pub tank: Ship,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ship {
    #[serde(rename = "Y")]
    pub y: super::enums::Rows,
    #[serde(rename = "X")]
    pub x: usize,
    #[serde(rename = "D")]
    pub direction: super::enums::Direction,
}

impl Layout {
    pub fn as_array(&self) -> Vec<Box<Ship>> {
        vec![
            Box::new(self.battle_ship),
            Box::new(self.attack_plane),
            Box::new(self.spy_plane),
            Box::new(self.fighter),
            Box::new(self.tank),
        ]
    }
}

impl Ship {
    pub fn get_coords(&self, length: usize) -> Vec<(usize, usize)> {
        let mut y = self.y as usize;
        let mut x = self.x - 1;
        let mut coords = vec![(y, x)];
        for _ in 1..length {
            match self.direction {
                Direction::North => y -= 1,
                Direction::South => y += 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            };
            coords.push((y, x));
        }
        coords
    }
}

/*



*/
use anyhow::Result;
use std::fs::File;
use std::io::Read;

pub fn serialize(json: String) -> Result<Vec<Layout>> {
    let ser: Vec<Layout> = serde_json::from_str(&json.as_str())?;

    Ok(ser)
}

pub fn serialize_file(path: &str) -> Result<Vec<Layout>> {
    let mut contents = String::new();

    File::open(path)?.read_to_string(&mut contents)?;

    Ok(serialize(contents)?)
}

#[cfg(test)]
mod test {
    use super::*;

    fn serialize() -> Vec<Layout> {
        let json = r#"[{"BattleShip":{"Y":"A","X":6,"D":"N"},"AttackPlane":{"Y":"B","X":2,"D":"S"},"SpyPlane":{"Y":"C","X":4,"D":"E"},"Fighter":{"Y":"G","X":5,"D":"W"},"Tank":{"Y":"I","X":1,"D":"N"}}]"#;
        super::serialize(json.to_string()).unwrap()
    }

    #[test]
    fn serialize_test() {
        let ser = serialize();

        assert_eq!(ser.len(), 1);
    }

    #[test]
    fn equalivance_test() {
        use super::super::enums::{Direction, Rows};

        let equal: Vec<Layout> = vec![Layout {
            battle_ship: Ship {
                x: 6,
                y: Rows::A,
                direction: Direction::North,
            },
            attack_plane: Ship {
                x: 2,
                y: Rows::B,
                direction: Direction::South,
            },
            spy_plane: Ship {
                x: 4,
                y: Rows::C,
                direction: Direction::East,
            },
            fighter: Ship {
                x: 5,
                y: Rows::G,
                direction: Direction::West,
            },
            tank: Ship {
                x: 1,
                y: Rows::I,
                direction: Direction::North,
            },
        }];

        let ser = serialize();

        println!("Serialized: {:?}\nInline: {:?}", ser, equal);

        assert_eq!(ser, equal);
    }
}
