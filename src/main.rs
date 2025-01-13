use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Terminal;
use std::io;
use std::process::Command;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode}; 
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let stdout = io::stdout();
   execute_command("clear"); 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut selected = 0;
    let options = vec!["Push", "Pull", "Logs", "Exit"];

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(80),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let items: Vec<ListItem> = options
                .iter()
                .enumerate()
                .map(|(i, o)| {
                    let style = if i == selected {
                        Style::default().fg(Color::Yellow)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Span::from(Span::styled(*o, style)))
                })
                .collect();
            let list = List::new(items).block(Block::default().borders(Borders::ALL).title("GitLens"));
            f.render_widget(list, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < options.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => match options[selected] {
                    "Push" => execute_command("git push"),
                    "Pull" => execute_command("git pull"),
                    "Logs" => execute_command("git log --oneline"),
                    "Exit" => break,
                    _ => {}
                },
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

fn execute_command(cmd: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
