// Inspired (and some initial code) from https://github.com/CleanCut/invaders
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use std::error::Error;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

const BOARD_HEIGHT: usize = 16;
const BOARD_WIDTH: usize = 20;

mod canvas;
mod render;
use canvas::*;
mod test;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let mut window = canvas::Window::movie_canvas();

    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut pause = false;
    let mut instant = Instant::now();
    let mut duration = 1000;
    render::print_window(&mut stdout, &window)?;
    // Game loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    KeyCode::Char('h') => {
                        window.left_one();
                    }
                    KeyCode::Char('l') => {
                        window.right_one();
                    }
                    KeyCode::Char('p') => {
                        if pause {
                            pause = false;
                        } else {
                            pause = true;
                        }
                    }
                    KeyCode::Char('+') => {
                        if duration < 5000 {
                            duration += 100;
                        }
                    }
                    KeyCode::Char('-') => {
                        if duration > 000 {
                            duration -= 100;
                        }
                    }
                    _ => {}
                }
            }
        }

        render::print_window(&mut stdout, &window)?;
        if pause {
            sleep(Duration::from_millis(duration));
            continue;
        }

        // Time to update yet?
        let refresh_time = Duration::from_millis(duration);
        if instant.elapsed() > refresh_time {
            let msg = format!("duration {duration}  ");
            render::print_msg(&mut stdout, &msg);
            render::print_window(&mut stdout, &window)?;
            instant = Instant::now();
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
