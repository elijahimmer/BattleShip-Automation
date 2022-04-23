use super::super::enums::Rows;
use super::cell::Cell;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Row {
    pub a: Cell,
    pub b: Cell,
    pub c: Cell,
    pub d: Cell,
    pub e: Cell,
    pub f: Cell,
    pub g: Cell,
    pub h: Cell,
    pub i: Cell,
}

impl Row {
    pub fn new() -> Row {
        Row {
            a: Cell::new(),
            b: Cell::new(),
            c: Cell::new(),
            d: Cell::new(),
            e: Cell::new(),
            f: Cell::new(),
            g: Cell::new(),
            h: Cell::new(),
            i: Cell::new(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            r#"{}  {}  {}  {}  {}  {}  {}  {}  {}"#,
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i
        )
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Index<usize> for Row {
    type Output = Cell;
    fn index(&self, i: usize) -> &Cell {
        match i {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            3 => &self.d,
            4 => &self.e,
            5 => &self.f,
            6 => &self.g,
            7 => &self.h,
            8 => &self.i,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, i: usize) -> &mut Cell {
        match i {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            3 => &mut self.d,
            4 => &mut self.e,
            5 => &mut self.f,
            6 => &mut self.g,
            7 => &mut self.h,
            8 => &mut self.i,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Index<&'_ str> for Row {
    type Output = Cell;
    fn index(&self, s: &str) -> &Cell {
        match s {
            "A" => &self.a,
            "B" => &self.b,
            "C" => &self.c,
            "D" => &self.d,
            "E" => &self.e,
            "F" => &self.f,
            "G" => &self.g,
            "H" => &self.h,
            "I" => &self.i,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<&'_ str> for Row {
    fn index_mut(&mut self, s: &str) -> &mut Cell {
        match s {
            "A" => &mut self.a,
            "B" => &mut self.b,
            "C" => &mut self.c,
            "D" => &mut self.d,
            "E" => &mut self.e,
            "F" => &mut self.f,
            "G" => &mut self.g,
            "H" => &mut self.h,
            "I" => &mut self.i,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl Index<Rows> for Row {
    type Output = Cell;
    fn index(&self, r: Rows) -> &Cell {
        match r {
            Rows::A => &self.a,
            Rows::B => &self.b,
            Rows::C => &self.c,
            Rows::D => &self.d,
            Rows::E => &self.e,
            Rows::F => &self.f,
            Rows::G => &self.g,
            Rows::H => &self.h,
            Rows::I => &self.i,
        }
    }
}

impl IndexMut<Rows> for Row {
    fn index_mut(&mut self, r: Rows) -> &mut Cell {
        match r {
            Rows::A => &mut self.a,
            Rows::B => &mut self.b,
            Rows::C => &mut self.c,
            Rows::D => &mut self.d,
            Rows::E => &mut self.e,
            Rows::F => &mut self.f,
            Rows::G => &mut self.g,
            Rows::H => &mut self.h,
            Rows::I => &mut self.i,
        }
    }
}
