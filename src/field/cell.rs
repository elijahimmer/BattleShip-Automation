use super::super::enums::ShipType;
use crossterm::ansi_support;
use crossterm::style::{style, Color, Stylize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    pub ship: ShipType,
    pub hit: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            ship: ShipType::None,
            hit: false,
        }
    }

    pub fn to_string(&self) -> String {
        self.to_string_color(ansi_support::supports_ansi())
    }

    pub fn to_string_color(&self, color: bool) -> String {
        if color {
            let string = match self.ship {
                ShipType::None => style("|"),
                ShipType::Battleship => style("B").with(Color::Cyan),
                ShipType::AttackPlane => style("A").with(Color::Green),
                ShipType::SpyPlane => style("S").with(Color::Yellow),
                ShipType::Fighter => style("F").with(Color::Magenta),
                ShipType::Tank => style("T").with(Color::Blue),
            };
            if self.hit {
                return string.with(Color::Red).to_string();
            }
            string.to_string()
        } else {
            if self.hit {
                return "X".to_string();
            }
            let string = match self.ship {
                ShipType::None => "|",
                ShipType::Battleship => "B",
                ShipType::AttackPlane => "A",
                ShipType::SpyPlane => "S",
                ShipType::Fighter => "F",
                ShipType::Tank => "T",
            };
            string.to_string()
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
