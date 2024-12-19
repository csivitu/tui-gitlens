use ratatui::{
    prelude::*,
    widgets::{self, *},
};

pub fn add_widgets_to_frame(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    let num_widgets = 10;

    let mut sidebar_widgets: Vec<widgets::Paragraph> = Vec::new();
    for i in 0..num_widgets {
        sidebar_widgets
            .push(Paragraph::new(format!("Hellow {i}")).block(Block::new().borders(Borders::ALL)));
    }

    let mut area_widgets: Vec<Constraint> = Vec::new();
    for _ in sidebar_widgets.iter() {
        area_widgets.push(Constraint::Ratio(1, num_widgets));
    }

    let inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(area_widgets)
        .split(outer_layout[0]);

    frame.render_widget(
        Paragraph::new("Vertical Sidebar").block(Block::new().borders(Borders::ALL)),
        outer_layout[1],
    );

    for (widget, area) in sidebar_widgets.iter().zip(inner_layout.iter()) {
        frame.render_widget(widget, *area);
    }
}
