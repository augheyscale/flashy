use std::sync::Arc;
use std::time::Duration;

use clap::Parser;
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use flash_async::run_async_flasher;
use flash_singlethreaded::{FlashPolled, ManyPoll, run_flash_sleep};
use flashlib::{LightId, TimeSource};
use ratatui::prelude::*;

use flash_tui::{event_loop, lights, time_source, tui_tracing};

#[derive(Parser, Debug)]
#[command(name = "flash-tui")]
#[command(about = "Arduino-like light simulator")]
struct Args {
    /// Choose the flashing mode
    #[arg(short, long, default_value = "polled")]
    mode: Mode,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Polled,
    Sleep,
    ManyPoll,
    Async,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Setup log buffer and tracing
    let log_buffer = Arc::new(tui_tracing::LogBuffer::new());
    let tracing_layer = tui_tracing::TuiTracingLayer::new(Arc::clone(&log_buffer));

    use tracing_subscriber::prelude::*;
    let subscriber = tracing_subscriber::registry().with(tracing_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    tracing::info!("TUI application started");
    tracing::info!("Mode: {:?}", args.mode);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    // Create our interfaces to work in the TUI
    let lights = lights::TuiLights::new();
    let time_source = time_source::TuiTimeSource::new();

    // Match on mode and set up handler, then run the common event loop
    match args.mode {
        Mode::Polled => {
            tracing::info!("Initializing FlashPolled mode");
            let mut flash_polled =
                FlashPolled::new(LightId::One, &time_source.now(), Duration::from_secs(1))
                    .ok_or_else(|| anyhow::anyhow!("Overflow"))?;
            event_loop::run_event_loop(&mut terminal, &lights, &log_buffer, || {
                flash_polled
                    .poll(&lights, &time_source.now())
                    .expect("Success");
            })
            .await?;
        }
        Mode::ManyPoll => {
            tracing::info!("Initializing ManyPoll mode");
            let mut many_poll = {
                let start_time = time_source.now();
                ManyPoll::new(vec![
                    FlashPolled::new(LightId::One, &start_time, Duration::from_millis(250))
                        .ok_or_else(|| anyhow::anyhow!("Overflow"))?,
                    FlashPolled::new(LightId::Two, &start_time, Duration::from_millis(500))
                        .ok_or_else(|| anyhow::anyhow!("Overflow"))?,
                    FlashPolled::new(LightId::Three, &start_time, Duration::from_millis(750))
                        .ok_or_else(|| anyhow::anyhow!("Overflow"))?,
                    FlashPolled::new(LightId::Four, &start_time, Duration::from_millis(1000))
                        .ok_or_else(|| anyhow::anyhow!("Overflow"))?,
                ])
            };

            event_loop::run_event_loop(&mut terminal, &lights, &log_buffer, || {
                many_poll
                    .poll(&lights, &time_source.now())
                    .expect("Success");
            })
            .await?;
        }
        Mode::Sleep => {
            tracing::info!("Initializing FlashSleep mode");
            // lights need to be shared between tasks, so we need to use Arc
            let lights = Arc::new(lights);
            let lights_for_task = lights.clone();
            let lights_for_task2 = lights.clone();

            // Spawn two background tasks for run_flash_sleep
            let start_time1 = time_source.now();
            let start_time2 = start_time1.clone();
            tokio::spawn(async move {
                run_flash_sleep(
                    lights_for_task.as_ref(),
                    &time_source::TuiTimeSource,
                    &start_time1,
                    LightId::One,
                    Duration::from_millis(250),
                )
            });
            tokio::spawn(async move {
                run_flash_sleep(
                    lights_for_task2.as_ref(),
                    &time_source::TuiTimeSource,
                    &start_time2,
                    LightId::Three,
                    Duration::from_millis(500),
                )
            });

            // No-op poll function since Sleep mode runs in background tasks
            event_loop::run_event_loop(&mut terminal, lights.as_ref(), &log_buffer, || {}).await?;
        }
        Mode::Async => {
            tracing::info!("Initializing AsyncFlashPolled mode");
            // Spawn background task for AsyncFlasher

            let now = time_source.now();

            let task1 = run_async_flasher(
                &now,
                LightId::One,
                Duration::from_millis(250),
                &lights,
                &time_source::TuiTimeSource,
            );
            let task2 = run_async_flasher(
                &now,
                LightId::Two,
                Duration::from_millis(500),
                &lights,
                &time_source::TuiTimeSource,
            );
            let task3 = run_async_flasher(
                &now,
                LightId::Three,
                Duration::from_millis(750),
                &lights,
                &time_source::TuiTimeSource,
            );
            let task4 = run_async_flasher(
                &now,
                LightId::Four,
                Duration::from_millis(1000),
                &lights,
                &time_source::TuiTimeSource,
            );

            // No-op poll function since AsyncPolled mode runs in background task
            let event_loop = event_loop::run_event_loop(&mut terminal, &lights, &log_buffer, || {});

            // wait for one task to complete
            tokio::select! {
                result = task1 => result?,
                result = task2 => result?,
                result = task3 => result?,
                result = task4 => result?,
                result = event_loop => result?,
            }
        }
    }

    tracing::info!("Shutting down TUI application");

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
