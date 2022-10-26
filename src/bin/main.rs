use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs::File;
use std::io;
use std::io::Write;
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use tui_chess::controls::{controls, Controls};
use tui_chess::game::GameState;
use tui_chess::ui::ui;

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, log_file: File) -> anyhow::Result<()> {
    let mut state = GameState::new(log_file);
    loop {
        terminal.draw(|f| ui(f, &state))?;
        match controls(&mut state)? {
            // write to log file and exit
            Controls::Quit => return Ok(()),
            Controls::Continue => {}
        }

        while let Ok(log) = state.logs_rx.try_recv() {
            state.messages.push(log.to_string());
            state.log(&log)?;
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let f = File::create("tui-chess.logs")?;
    let mut f_clone = f.try_clone()?;
    let res = run_app(&mut terminal, f).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        let msg = format!("ERROR: {:?}\n", err);
        writeln!(f_clone, "{msg}")?;
        f_clone.flush()?;
        eprintln!("{msg}");
        return Ok(());
    }
    f_clone.flush()?;
    println!("Thank you for using tui_chess <3\n\nPress any key to exit...");

    Ok(())
}
