use super::field::Field;
use super::layouts::Layout;
use anyhow::Result;

static mut J: usize = 0;

pub fn load_layout(layout: Box<Layout>, field: Option<Box<Field>>) -> Result<Box<Field>> {
    let mut f = field.unwrap_or(Box::new(Field::new()));
    let mut i: usize = 0;
    unsafe {
        J += 1;
    }

    for ship in layout.as_array() {
        let coords = ship.get_coords([5, 4, 3, 3, 2][i]);
        //println!("{}: {}: {:#?}", unsafe { J }, i, ship);

        for coord in coords {
            //println!("x: {:?}\ny: {:?}", coord.1, super::enums::int2row(coord.0));

            if coord.0 > 8 || coord.1 > 8 {
                return Err(anyhow::anyhow!(
                    "Ship is out of bounds at {:?}, length {:?}",
                    coord,
                    [5, 4, 3, 3, 2][i]
                ));
            }
            if f[coord.0][coord.1].ship != super::enums::ShipType::None {
                return Err(anyhow::anyhow!("Ship already exists at {:?}", coord));
            }

            f[coord.0][coord.1] = super::field::cell::Cell {
                ship: [
                    super::enums::ShipType::Battleship,
                    super::enums::ShipType::AttackPlane,
                    super::enums::ShipType::SpyPlane,
                    super::enums::ShipType::Fighter,
                    super::enums::ShipType::Tank,
                ][i],
                hit: false,
            };
        }

        i += 1;
    }

    Ok(f)
}
