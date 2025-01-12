use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;
use std::io;
use std::process::Command;
use crossterm::event::{self, Event, KeyCode};
use std::collections::HashSet;

struct App {
    files: Vec<String>,
    selected: usize,
    staged: HashSet<String>,
    commit_msg: String,
    commit_desc: String,
    committing: bool,
    editing_desc: bool,
}

impl App {
    fn new() -> Self {
        let mut initially_staged=HashSet::new();
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .expect("Failed to run git status");
        let files = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| {
            let file = line[3..].to_string();
            if line.starts_with("A ") || line.starts_with("M ") {
                initially_staged.insert(file.clone());
            }
            file
            })
            .collect();

        Self {
            files,
            selected: 0,
            staged: initially_staged,
            commit_msg: String::new(),
            commit_desc: String::new(),
            committing: false,
            editing_desc: false,
        }
    }

    fn next(&mut self) {
        if self.selected < self.files.len() - 1 {
            self.selected += 1;
        }
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn toggle_stage(&mut self) {
        let file = &self.files[self.selected];
        if self.staged.contains(file) {
            self.staged.remove(file);
            if Command::new("git")
                .args(["reset","HEAD", "--", file])
                .output()
                .is_err()
            {
                eprintln!("Failed to unstage file");
            }
        } else {
            self.staged.insert(file.clone());
            if Command::new("git")
                .args(["add", file])
                .output()
                .is_err()
            {
                eprintln!("Failed to stage file");
            }
        }
    }

    fn commit(&self) {
        if Command::new("git")
            .args(["commit", "-m", &self.commit_msg, "-m", &self.commit_desc])
            .output()
            .is_err()
        {
            eprintln!("Failed to commit");
        }
    }
}
fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [Constraint::Percentage(70), Constraint::Percentage(30)].as_ref(),
        )
        .split(f.area());

    let file_items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(_i, file)| {
            let style = if app.staged.contains(file) {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(Span::from(file.clone())).style(style)
        })
        .collect();

    let files = List::new(file_items)
        .block(Block::default().borders(Borders::ALL).title("Changed Files"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("=> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(app.selected));
    f.render_stateful_widget(files, chunks[0], &mut state);

    if app.committing {
        let popup_area = ratatui::layout::Rect {
            x: (f.area().width - 50) / 2,
            y: (f.area().height - 10) / 2,
            width: 50,
            height: 10,
        };

        let padded_area = ratatui::layout::Rect {
            x: popup_area.x + 1,
            y: popup_area.y + 1,
            width: popup_area.width - 2,
            height: popup_area.height - 2,
        };

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title("Commit Message");
        f.render_widget(popup_block, popup_area);

        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
            [Constraint::Length(3), Constraint::Length(3)].as_ref(),
            )
            .split(padded_area);

        let msg = Paragraph::new(app.commit_msg.as_str())
            .block(Block::default()
            .borders(Borders::ALL)
            .border_style(if app.editing_desc{Style::default()} else {Style::default().fg(Color::Green)})
            .title(if app.editing_desc { "Message" } else { "Message (Active)" }));

        f.render_widget(msg, popup_chunks[0]);

        let desc = Paragraph::new(app.commit_desc.as_str())
            .block(Block::default()
            .borders(Borders::ALL)
            .border_style(if app.editing_desc{Style::default().fg(Color::Green)} else {Style::default()})
            .title(if app.editing_desc { "Description (Active)" } else { "Description" }));

        f.render_widget(desc, popup_chunks[1]);

        if app.editing_desc {
            f.set_cursor_position(
                ratatui::layout::Position::new(popup_chunks[1].x + app.commit_desc.len() as u16 + 1, popup_chunks[1].y + 1)
            );
        } else {
            f.set_cursor_position(
                ratatui::layout::Position::new(popup_chunks[0].x + app.commit_msg.len() as u16 + 1, popup_chunks[0].y + 1)
            );
        }
    }
}

fn main() -> Result<(), io::Error> {
    crossterm::terminal::enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    terminal.clear()?;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if app.committing {
                match key.code {
                    KeyCode::Char(c) => {
                        if app.editing_desc {
                            app.commit_desc.push(c);
                        } else {
                            app.commit_msg.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.editing_desc {
                            app.commit_desc.pop();
                        } else {
                            app.commit_msg.pop();
                        }
                    }
                    KeyCode::Tab => app.editing_desc = !app.editing_desc,
                    KeyCode::Esc => app.committing = false,
                    KeyCode::Enter => {
                        app.commit();
                        break;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char(' ') => app.toggle_stage(),
                    KeyCode::Enter => app.committing = true,
                    _ => {}
                }
            }
        }
    }

    terminal.clear()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
