use std::{
    io::{Cursor, Stdout, Write, stdout},
    string,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, poll, read},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size},
};

struct World {
    player_c: u16,
    player_l: u16,
    map: Vec<(u16, u16)>,
    max_l: u16,
    max_c: u16,
}

fn draw(mut sc: &Stdout, world: &World) -> std::io::Result<()> {
    sc.queue(Clear(ClearType::All))?;

    for l in 0..world.map.len() {
        sc.queue(MoveTo(0, l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;

        sc.queue(MoveTo(world.map[l].1, l as u16))?;
        sc.queue(Print("+".repeat((world.max_c - world.map[l].1) as usize)))?;
    }
    sc.queue(MoveTo(world.player_c, world.player_l))?;
    sc.queue(Print("P"))?;
    sc.flush()?;

    Ok(())
}

fn physic(mut world : World) -> Result<World>{
    asad;
    asda;
    Ok(world)   
}

fn main() -> std::io::Result<()> {
    let mut sc = stdout();
    let (max_c, max_l) = size().unwrap();
    enable_raw_mode()?;
    let mut world: World = World {
        player_c: max_c / 2,
        player_l: max_l - 1,
        map: vec![((max_c / 2) - 10, (max_c / 2) + 10); max_l as usize],
        max_c: max_c,
        max_l: max_l,
    };
    draw(&sc, &world)?;
    sc.execute(Hide)?;
    loop {
        //ready and apply keyboard
        if poll(Duration::from_millis(500))? {
            let key = read().unwrap();
            match key {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => {
                        if world.player_l > 1 {
                            world.player_l -= 1;
                        }
                    }
                    KeyCode::Char('s') => {
                        if world.player_l < max_l {
                            world.player_l += 1;
                        }
                    }
                    KeyCode::Char('d') => {
                        if (world.player_c < max_c - 1) {
                            world.player_c += 1;
                        }
                    }
                    KeyCode::Char('a') => {
                        if (world.player_c > 1) {
                            world.player_c -= 1;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
        }

        //physics()

        draw(&sc, &world)?;
    }
    sc.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}

//49:10
