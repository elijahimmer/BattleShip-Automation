use super::super::enums::Rows;
use std::ops::{Index, IndexMut};

impl Index<usize> for super::Field {
    type Output = super::row::Row;
    fn index(&self, i: usize) -> &super::row::Row {
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

impl IndexMut<usize> for super::Field {
    fn index_mut(&mut self, i: usize) -> &mut super::row::Row {
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

impl Index<&'_ str> for super::Field {
    type Output = super::row::Row;
    fn index(&self, s: &str) -> &super::row::Row {
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

impl IndexMut<&'_ str> for super::Field {
    fn index_mut(&mut self, s: &str) -> &mut super::row::Row {
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

impl Index<&'_ Rows> for super::Field {
    type Output = super::row::Row;
    fn index(&self, r: &Rows) -> &super::row::Row {
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

impl IndexMut<&'_ Rows> for super::Field {
    fn index_mut(&mut self, r: &Rows) -> &mut super::row::Row {
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
