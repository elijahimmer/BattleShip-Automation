pub mod index;
pub mod print;

pub mod cell;
pub mod row;

mod test;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Field {
    pub a: row::Row,
    pub b: row::Row,
    pub c: row::Row,
    pub d: row::Row,
    pub e: row::Row,
    pub f: row::Row,
    pub g: row::Row,
    pub h: row::Row,
    pub i: row::Row,
}

impl Field {
    pub fn new() -> Field {
        return Field {
            a: row::Row::new(),
            b: row::Row::new(),
            c: row::Row::new(),
            d: row::Row::new(),
            e: row::Row::new(),
            f: row::Row::new(),
            g: row::Row::new(),
            h: row::Row::new(),
            i: row::Row::new(),
        };
    }

    #[allow(dead_code)]
    pub fn get(&self, y: usize, x: usize) -> &cell::Cell {
        &self[y][x]
    }

    #[allow(dead_code)]
    pub fn set(&mut self, y: usize, x: usize, value: cell::Cell) {
        self[y][x] = value;
    }
}
