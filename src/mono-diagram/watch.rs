use std::{
    fs::{metadata, Metadata},
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use anyhow::{Error, Result};
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, KeyCode, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{self, Clear},
};

use crate::{
    parser::{parse, write},
    utils::add_prefix,
};

pub fn watch(file: &str, prefix: Option<String>) -> Result<()> {
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let mut is_running = true;
    let mut init_metadata;
    while is_running {
        execute!(stdout, Clear(terminal::ClearType::All), MoveTo(0, 0))?;
        // stdout.flush()?;
        let result = parse(file).and_then(|d| write(&d));
        match result {
            Ok(d) => {
                println!(
                    "{}",
                    add_prefix(
                        String::from_utf8_lossy(&d).to_string(),
                        &prefix.clone().unwrap_or("".to_string())
                    )
                );
            }
            Err(e) => {
                println!("{}", e)
            }
        }

        // queue!(
        //     stdout,
        //     MoveTo(0, 0),
        //     Print(add_prefix(
        //         String::from_utf8_lossy(&result).to_string(),
        //         &prefix.clone().unwrap_or("".to_string())
        //     ))
        // )?;
        // sleep(Duration::from_secs_f32(0.5));

        stdout.flush()?;
        init_metadata =
            metadata(file).map_err(|_| Error::msg("fail to fetch metadata of the file"))?;

        while !file_changed(file, &init_metadata)? {
            if event::poll(Duration::ZERO)? {
                match event::read()? {
                    event::Event::Key(e) => {
                        if e.code == KeyCode::Esc
                            || (e.modifiers == KeyModifiers::CONTROL
                                && e.code == KeyCode::Char('c'))
                        {
                            is_running = false;
                            break;
                        }
                    }
                    event::Event::Resize(_, _) => {
                        break;
                    }
                    _ => {}
                };
            }
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn file_changed(file: &str, prev_metadata: &Metadata) -> Result<bool> {
    let cur_metadata =
        metadata(file).map_err(|_| Error::msg("fail to fetch metadata of the file"))?;
    if cur_metadata.len() != prev_metadata.len() {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
