use ratatui::{prelude::*, widgets::*};

pub fn title(text: String) -> Paragraph<'static> {
    return Paragraph::new(text)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double),
        )
        .bg(Color::Blue)
        .alignment(Alignment::Center);
}
