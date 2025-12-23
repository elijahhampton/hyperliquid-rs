use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use rhyperliquid::{ tui::{App, Event, EventHandler}, HyperliquidClientBuilder};
use std::{env, io};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let signer = env::var("HL_PRIVATE_KEY").expect("HL_PRIVATE_KEY env var is missing");

    let client = HyperliquidClientBuilder::new()
        .testnet()
        .with_wallet(signer.parse().expect(""))
        .build()?;

    let mut terminal = setup_terminal()?;
    let mut app = App::new(client);

    let (tx, mut rx) = mpsc::channel(100);
    let event_handler = EventHandler::new(tx);

    tokio::spawn(async move {
        event_handler.run().await;
    });

    loop {
        terminal.draw(|f| rhyperliquid::tui::ui::render(f, &app))?;

        if let Some(event) = rx.recv().await {
            match event {
                Event::Key(key) => {
                    app.handle_key(key.code).await;
                }
                Event::Tick => {
                    app.handle_tick().await;
                }
            }

            if app.should_quit {
                break;
            }
        }
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
