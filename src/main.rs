mod enums;
mod field;
mod layouts;
mod load;

use anyhow::Result;
use crossterm::{
    cursor, execute, queue, style,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

fn main() -> Result<()> {
    ctrlc::set_handler(|| end(&mut io::stdout(), true).unwrap())?;

    let mut stdout = io::stdout();
    let layout_vec = layouts::serialize_file("layout.json").unwrap();

    queue!(stdout, EnterAlternateScreen)?;
    queue!(stdout, cursor::Hide)?;

    for i in 0..layout_vec.len() {
        let battle_field = load::load_layout(Box::new(layout_vec[i]), None)?;

        queue!(
            stdout,
            style::Print(format!(
                "{:?}-{}\n",
                enums::int2row(num_integer::div_rem(i, 9).0),
                num_integer::div_rem(i, 9).1 + 1
            )),
        )?;
        execute!(stdout, style::Print(battle_field))?;

        loop {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(crossterm::event::KeyEvent { .. }) => break,
                _ => {}
            }
        }
        queue!(stdout, cursor::MoveUp(23))?;
        queue!(stdout, cursor::MoveLeft(60))?;
    }

    end(&mut stdout, false)?;

    Ok(())
}

fn end(stdout: &mut io::Stdout, exit: bool) -> Result<()> {
    queue!(stdout, cursor::Show)?;
    queue!(stdout, LeaveAlternateScreen)?;

    if exit {
        std::process::exit(0);
    }

    Ok(())
}
