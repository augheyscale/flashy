use flashlib::{LightId, Lights, OnOff};
use ratatui::prelude::*;
use ratatui::widgets::*;
use tracing::Level;

use crate::tui_tracing;

/// Renders the lights and the log window in the TUI.
///
/// This function draws the complete UI layout:
/// - A horizontal row of five light indicators at the top
/// - A log window below showing tracing output
///
/// Each light is displayed as a colored box (green for on, dark gray for off)
/// with its name and current state.
///
/// The log window shows the most recent log entries with color-coded log levels.
///
/// # Arguments
///
/// * `frame` - The ratatui frame to render into
/// * `lights` - The lights interface to query for current states
/// * `log_buffer` - The log buffer containing tracing entries to display
pub fn render_lights(
    frame: &mut Frame,
    lights: &impl Lights,
    log_buffer: &tui_tracing::LogBuffer,
) -> anyhow::Result<()> {
    let states = LightId::all().map(|light_id| lights.get_state(light_id));
    let light_names = ["Light 1", "Light 2", "Light 3", "Light 4", "Light 5"];

    // Split the area into lights area, and log area
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Lights
            Constraint::Min(5),    // Log window (at least 5 lines)
        ])
        .split(frame.area());

    // Render lights
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(12),
        ])
        .split(main_chunks[0]);

    for (i, (chunk, state)) in chunks.iter().zip(states.into_iter()).enumerate() {
        let state = state?;
        let color = match state {
            OnOff::On => Color::Green,
            OnOff::Off => Color::DarkGray,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(color))
            .style(Style::default().bg(color));

        let text = vec![
            Line::from(light_names[i]),
            Line::from(""),
            Line::from(match state {
                OnOff::On => "  ON  ",
                OnOff::Off => " OFF  ",
            }),
        ];

        let paragraph = Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, *chunk);
    }

    // Render log window
    let log_entries = log_buffer.get_entries();
    let log_lines: Vec<Line> = log_entries
        .iter()
        .rev() // Show most recent first
        .take(main_chunks[1].height as usize - 2) // Account for borders
        .map(|entry| {
            let level_color = match entry.level {
                Level::ERROR => Color::Red,
                Level::WARN => Color::Yellow,
                Level::INFO => Color::Cyan,
                Level::DEBUG => Color::Blue,
                Level::TRACE => Color::Magenta,
            };

            let level_str = match entry.level {
                Level::ERROR => "ERROR",
                Level::WARN => "WARN ",
                Level::INFO => "INFO ",
                Level::DEBUG => "DEBUG",
                Level::TRACE => "TRACE",
            };

            Line::from(vec![
                Span::styled(
                    format!("[{}] ", level_str),
                    Style::default().fg(level_color),
                ),
                Span::raw(&entry.message),
            ])
        })
        .collect();

    let log_paragraph = Paragraph::new(log_lines)
        .block(Block::default().borders(Borders::ALL).title("Tracing Log"))
        .wrap(Wrap { trim: true })
        .scroll((0, 0));
    frame.render_widget(log_paragraph, main_chunks[1]);
    Ok(())
}
