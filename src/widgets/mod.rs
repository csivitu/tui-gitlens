use ratatui::widgets::ListState;

pub struct Entry {
    text: String,
}

impl Entry {
    pub fn new(text: String) -> Self {
        return Entry { text };
    }
}

pub struct EntryList {
    items: Vec<Entry>,
    state: ListState,
}

impl EntryList {
    pub fn new() -> Self {
        return EntryList {
            items: Vec::new(),
            state: ListState::default(),
        };
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }
}

pub struct WidgetState {
    pub title: String,
    pub branches: EntryList,
    pub files: EntryList,
}

impl WidgetState {
    pub fn new(title: String) -> Self {
        return WidgetState {
            title,
            branches: EntryList::new(),
            files: EntryList::new(),
        };
    }
}
