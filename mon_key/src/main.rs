use crossterm::{
    event::{self, read, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{stdout, Write};

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;

    loop {
        if let Event::Key(key_event) = read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char(c) => {
                    write!(stdout, "{}", c)?;
                    stdout.flush()?;
                },
                _ => {}
            }
        }
    }

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()
}
