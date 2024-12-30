pub struct State {
    pub num_widgets: u32,
    pub selected_widget: u32,
    pub delay: u64,
    pub running: bool,
}

impl State {
    pub fn new(num_widgets: u32) -> Self {
        return State {
            num_widgets,
            selected_widget: 0,
            delay: 1000,
            running: true,
        };
    }
}
