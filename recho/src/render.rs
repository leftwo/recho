use crossterm::{
    cursor,
    style::{self, Colorize},
    QueueableCommand,
};
use std::error::Error;
use std::io::Stdout;

use super::*;

pub fn print_msg(stdout: &mut Stdout, msg: &str) {
    stdout
        .queue(cursor::MoveTo(BOARD_WIDTH as u16 + 3, 7))
        .unwrap();
    stdout
        .queue(style::PrintStyledContent(msg.black()))
        .unwrap();
}

// Print the current window on the given canvas
#[allow(clippy::needless_range_loop)]
pub fn print_window(
    stdout: &mut Stdout,
    window: &Window,
) -> Result<(), Box<dyn Error>> {
    stdout.queue(cursor::MoveTo(BOARD_WIDTH as u16 + 3, 0))?;
    stdout.queue(style::PrintStyledContent("q to quit".black()))?;
    stdout.queue(cursor::MoveTo(BOARD_WIDTH as u16 + 3, 1))?;
    stdout.queue(style::PrintStyledContent("a Left".black()))?;
    stdout.queue(cursor::MoveTo(BOARD_WIDTH as u16 + 3, 2))?;
    stdout.queue(style::PrintStyledContent("d Right".black()))?;
    stdout.queue(cursor::MoveTo(BOARD_WIDTH as u16 + 3, 3))?;
    stdout.queue(style::PrintStyledContent("s Down".black()))?;

    // border
    for x in 0..BOARD_WIDTH {
        stdout.queue(cursor::MoveTo((x + 1) as u16, 0))?;
        stdout.queue(style::PrintStyledContent("T".black()))?;
    }
    for y in 0..BOARD_HEIGHT {
        stdout.queue(cursor::MoveTo(0, (y + 1) as u16))?;
        stdout.queue(style::PrintStyledContent("L".black()))?;
        for x in 0..BOARD_WIDTH {
            stdout.queue(cursor::MoveTo((x + 1) as u16, (y + 1) as u16))?;
            let window_x = window.current_x + x;
            if window.canvas[window_x][y] == 1 {
                stdout.queue(style::PrintStyledContent("b".black()))?;
            } else {
                stdout.queue(style::PrintStyledContent(" ".black()))?;
            }
        }
        stdout
            .queue(cursor::MoveTo((BOARD_WIDTH + 1) as u16, (y + 1) as u16))?;
        stdout.queue(style::PrintStyledContent("R".black()))?;
    }
    for x in 0..BOARD_WIDTH {
        stdout
            .queue(cursor::MoveTo((x + 1) as u16, (BOARD_HEIGHT + 1) as u16))?;
        stdout.queue(style::PrintStyledContent("B".black()))?;
    }
    Ok(())
}
