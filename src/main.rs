use chrono::{Datelike, Local, Timelike};
use std::{
    io::{stdout, Result},
    time::SystemTime,
    u128,
};
use tui_big_text::BigText;

const MSG_BOX_TIMEOUT: u32 = 2000;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::Paragraph,
    Terminal,
};

struct Timer {
    start_time_ms: SystemTime,
    rem_time_ms: u32,
    finished: bool,
}

fn main() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    let mut timer = Timer {
        start_time_ms: SystemTime::now(),
        rem_time_ms: 0,
        finished: true,
    };

    loop {
        let time = Local::now();

        terminal.draw(|frame| {
            if !timer.finished {
                if timer.start_time_ms.elapsed().unwrap().as_millis() < timer.rem_time_ms as u128 {
                    timer = Timer {
                        start_time_ms: timer.start_time_ms,
                        finished: false,
                        rem_time_ms: timer.rem_time_ms,
                    };
                } else {
                    timer = Timer {
                        start_time_ms: SystemTime::now(),
                        finished: true,
                        rem_time_ms: 0,
                    };
                }
            }

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(30),
                    Constraint::Percentage(45),
                    Constraint::Percentage(25),
                ])
                .split(frame.area());

            let msg_box = Paragraph::new("Press q to quit!").blue();

            if !timer.finished {
                frame.render_widget(msg_box, layout[0]);
            }

            let time_text = BigText::builder()
                .centered()
                .pixel_size(tui_big_text::PixelSize::Full)
                .style(Style::new().light_green())
                .lines(vec![format!(
                    "{:02}:{:02}:{:02}",
                    time.hour(),
                    time.minute(),
                    time.second()
                )
                .light_green()
                .into()])
                .build();
            frame.render_widget(time_text, layout[1]);

            let day_text = BigText::builder()
                .centered()
                .pixel_size(tui_big_text::PixelSize::Quadrant)
                .style(Style::new().magenta())
                .lines(vec![format!(
                    "{}, {:02}-{:02}-{:02}",
                    time.weekday(),
                    time.year(),
                    time.month(),
                    time.day()
                )
                .magenta()
                .into()])
                .build();

            frame.render_widget(day_text, layout[2]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        break;
                    } else {
                        timer = Timer {
                            start_time_ms: SystemTime::now(),
                            rem_time_ms: MSG_BOX_TIMEOUT,
                            finished: false,
                        };
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
