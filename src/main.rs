mod app;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use figlet_rs::FIGfont;
use std::{io, time::Duration};
use tui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Mode};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "IbIS Benchmark")]
struct Args {
    /// Minutes of work
    #[arg(short, long, default_value_t = 30)]
    work_time: u64,

    /// Minutes of break
    #[arg(short, long, default_value_t = 5)]
    break_time: u64,
}

fn format_time(d: Duration) -> String {
    let standard_font = FIGfont::standard().unwrap();
    let secs = d.as_secs();
    let min = secs / 60;
    let sec = secs % 60;
    let figlet = standard_font
        .convert(format!("{:02}:{:02}", min, sec).as_str())
        .unwrap();
    format!("{}", figlet)
}

fn main() -> Result<(), io::Error> {
    let Args {
        work_time,
        break_time,
    } = Parser::parse();
    let standard_font = FIGfont::standard().unwrap();
    let work_title = format!("{}", standard_font.convert("Work").unwrap());
    let break_title = format!("{}", standard_font.convert("Break").unwrap());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(work_time, break_time);
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let block = Block::default()
                .title("Pomodors Timer")
                .borders(Borders::ALL);

            let mode_text = match app.mode {
                Mode::Work => work_title.clone(),
                Mode::Break => break_title.clone(),
            };

            let time_text = format_time(app.remaining);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(3)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Length(time_text.lines().count() as u16 + 1),
                    Constraint::Length(2),
                    Constraint::Length(mode_text.lines().count() as u16),
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Percentage(30),
                ])
                .split(size);

            let mode_color = match app.mode {
                Mode::Work => Color::Red,
                Mode::Break => Color::Green,
            };

            let mode_paragraph = Paragraph::new(mode_text)
                .style(Style::default().fg(mode_color).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);

            let time_paragraph = Paragraph::new(time_text)
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center);
            let of = Paragraph::new(String::from("of"))
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Gray));

            let help = Paragraph::new(match app.mode {
                Mode::Work => "[Space] Start/Pause   [r] Start break   [q] Quit",
                Mode::Break => "[Space] Start/Pause   [r] Start work    [q] Quit",
            })
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));

            f.render_widget(block, size);
            f.render_widget(time_paragraph, chunks[1]);
            f.render_widget(of, chunks[2]);

            f.render_widget(mode_paragraph, chunks[3]);
            f.render_widget(help, chunks[4]);
        })?;

        if event::poll(tick_rate)?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => app.switch_mode(),
                KeyCode::Char(' ') => app.toggle(),
                _ => {}
            }
        }

        app.tick();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
