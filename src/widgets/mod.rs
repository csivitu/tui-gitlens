use ratatui::{
    prelude::*,
    widgets::{self, *},
};

use crate::app_state::State;

pub fn add_widgets_to_frame(frame: &mut Frame, app: &State) {
    let title_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(8), Constraint::Percentage(92)])
        .split(frame.area());

    let outer_text_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(title_layout[1]);

    let mut sidebar_widgets: Vec<widgets::Paragraph> = Vec::new();
    for i in 0..app.num_widgets {
        sidebar_widgets
            .push(Paragraph::new(format!("Hellow {i}")).block(Block::new().borders(Borders::ALL)));
    }

    let mut area_widgets: Vec<Constraint> = Vec::new();
    for _ in sidebar_widgets.iter() {
        area_widgets.push(Constraint::Ratio(1, app.num_widgets));
    }

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(area_widgets)
        .split(outer_text_layout[0]);

    frame.render_widget(
        Paragraph::new("Vertical Sidebar").block(Block::new().borders(Borders::ALL)),
        outer_text_layout[1],
    );

    frame.render_widget(
        Paragraph::new("GITLENS-TUI")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double),
            )
            .bg(Color::Blue)
            .alignment(Alignment::Center),
        title_layout[0],
    );

    for (widget, area) in sidebar_widgets.iter().zip(inner_layout.iter()) {
        frame.render_widget(widget, *area);
    }
}
