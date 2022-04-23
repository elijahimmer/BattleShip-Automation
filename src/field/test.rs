#[cfg(test)]
use super::super::enums::ShipType;

#[test]
fn make_test() {
    super::Field::new();
}

#[test]
fn get_test() {
    let field = super::Field::new();
    let new_cell = super::cell::Cell::new();

    for y in 0..9 {
        for x in 0..9 {
            assert_eq!(field.get(y, x), &new_cell);
        }
    }
}

#[test]
fn set_test() {
    let mut field = super::Field::new();
    let new_cell = super::cell::Cell {
        ship: ShipType::Battleship,
        hit: true,
    };

    for y in 0..9 {
        for x in 0..9 {
            field.set(y, x, new_cell.clone());
        }
    }

    println!("Passed Seting Test");

    for y in 0..9 {
        for x in 0..9 {
            assert_eq!(field.get(y, x), &new_cell);
        }
    }

    println!("Passed Setting Confirmation Test");
}

#[test]
fn index_test() {
    let mut field = super::Field::new();
    let new_cell = super::cell::Cell::new();

    for y in 0..9 {
        for x in 0..9 {
            assert_eq!(field[y][x], new_cell);
        }
    }

    println!("Passed Index Get");

    let changed_cell = super::cell::Cell {
        ship: ShipType::Battleship,
        hit: true,
    };

    for y in 0..9 {
        for x in 0..9 {
            field[y][x] = changed_cell.clone();
        }
    }

    println!("Passed Index Set");

    for y in 0..9 {
        for x in 0..9 {
            assert_eq!(field[y][x], changed_cell);
        }
    }

    println!("Passed Index Set Comfirmation");
}

#[test]
fn print_test() {
    assert_eq!(
        super::Field::new().to_string(),
        "   1  2  3  4  5  6  7  8  9
A  |  |  |  |  |  |  |  |  |
B  |  |  |  |  |  |  |  |  | 
C  |  |  |  |  |  |  |  |  |
D  |  |  |  |  |  |  |  |  |
E  |  |  |  |  |  |  |  |  |
F  |  |  |  |  |  |  |  |  |
G  |  |  |  |  |  |  |  |  |
H  |  |  |  |  |  |  |  |  |
I  |  |  |  |  |  |  |  |  |"
    );
}
