use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEventKind};
use crossterm::terminal::disable_raw_mode;
use flashlib::Lights;
use ratatui::prelude::*;

use crate::tui_tracing;
use crate::ui;

/// Generic event loop function for the TUI application.
///
/// This function runs the main event loop that:
/// - Polls the provided handler function to update application state
/// - Renders the UI (lights and log window)
/// - Handles keyboard input (q, Esc, or Ctrl+C to quit)
/// - Sleeps briefly to avoid busy-waiting
///
/// The loop continues until the user quits via keyboard input.
pub async fn run_event_loop<F>(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    lights: &impl Lights,
    log_buffer: &tui_tracing::LogBuffer,
    mut poll_fn: F,
) -> anyhow::Result<()>
where
    F: FnMut(),
{
    let mut should_quit = false;

    loop {
        // Poll the handler logic
        poll_fn();

        // Render
        terminal.draw(|frame| {
            ui::render_lights(frame, lights, log_buffer);
        })?;

        // Handle events
        if event::poll(Duration::from_millis(1))? {
            if let CrosstermEvent::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            should_quit = true;
                        }
                        KeyCode::Char('c')
                            if key.modifiers.contains(event::KeyModifiers::CONTROL) =>
                        {
                            should_quit = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        if should_quit {
            break;
        }

        // Small sleep to avoid busy-waiting
        tokio::time::sleep(Duration::from_millis(16)).await;
    }

    // Terminal cleanup
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
