use std::{
    io::{Stdout, Write, stdout},
    string,
    time::Duration,
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::MoveTo,
    event::{self, Event, poll, read},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, disable_raw_mode, enable_raw_mode, size},
};

struct World {
    player_c: u16,
    player_l: u16,
}

fn draw(mut sc: &Stdout, world: &World) {
    sc.queue(MoveTo(world.player_c, world.player_l));
    sc.queue(Print("P"));
    sc.flush();
}

fn main() -> std::io::Result<()> {
    //init screen
    //init game

    let sc = stdout();
    let (max_c, max_l) = size().unwrap();
    enable_raw_mode();
    let mut world: World = World {
        player_c: max_c / 2,
        player_l: max_l - 1,
    };
    draw(&sc, &world);

    loop {
        //ready and apply keyboard

        if poll(Duration::from_millis(500))? {
            let key = read().unwrap();
            match key {
                Event::Key(event) => {
                    println!("{:?}", event)
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        //physics()

        // draw
    }
    disable_raw_mode();
    Ok(())
}

//29:54