use crate::widgets;
pub struct State {
    pub num_widgets: u32,
    pub selected_widget: u32,
    pub delay: u64,
    pub running: bool,
    pub widget_states: widgets::WidgetState,
}

impl State {
    pub fn new(num_widgets: u32, title: String) -> Self {
        return State {
            num_widgets,
            selected_widget: 0,
            delay: 1000,
            running: true,
            widget_states: widgets::WidgetState::new(title),
        };
    }
}
