use app::CurrentMode;
use crossterm::{
    event::{self, KeyCode}, execute, terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen, SetTitle,
    }, ExecutableCommand
};
use ratatui::prelude::*;
use std::io::{stdout, Result};
mod app;
mod ui;
use crate::ui::ui;
use crate::app::App;

pub const LENGTH: u16 = 3;
//--------------------------
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
            }
            match app.current_mode {
                app::CurrentMode::Normal => match key.code {
                    KeyCode::Char('q') => app.current_mode = CurrentMode::Exiting,
                    KeyCode::Char('i') => app.current_mode = CurrentMode::Insert,

                    KeyCode::Char('h') => if (app.cursorpos.0 - 1) > 0 {
                        app.cursorpos.0 -= 1
                    }
                    KeyCode::Char('j') => if (app.cursorpos.1 + 1) < (terminal.get_frame().size().bottom() - LENGTH - 1) {
                        app.cursorpos.1 += 1
                    }
                    KeyCode::Char('k') => if (app.cursorpos.1 - 1) > LENGTH {
                        app.cursorpos.1 -= 1
                    }
                    KeyCode::Char('l') => if (app.cursorpos.0 + 1) < (terminal.get_frame().size().right() - 1) {
                        app.cursorpos.0 += 1
                    }
                    _ => {}
                }
                app::CurrentMode::Insert => match key.code {
                    KeyCode::Char(value) => {
                        if (app.cursorpos.0 + 1) < (terminal.get_frame().size().right() - 1) {
                            app.cursorpos.0 += 1
                        }
                        app.input.push(value);
                    }
                    KeyCode::Backspace => {
                        if !app.input.is_empty() && ((app.cursorpos.0 - 1) > 0) {
                            app.cursorpos.0 -= 1
                        }
                        app.input.pop();
                    }
                    KeyCode::Esc => app.current_mode = CurrentMode::Normal,
                    _ => {}
                }
                app::CurrentMode::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen, SetTitle("etui"))?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app);

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
