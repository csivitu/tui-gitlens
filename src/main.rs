mod app_state;
mod key_handler;
mod widgets;

use app_state::State;
use key_handler::read_key;
use ratatui::DefaultTerminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    match terminal.clear() {
        Err(error) => panic!("Error: {error:?}"),
        Ok(_) => (),
    };
    let app_result = run(terminal);
    ratatui::restore();
    return app_result;
}

fn run(mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let mut app: State = app_state::State::new(2);
    while app.running {
        match terminal.draw(|frame| {
            widgets::add_widgets_to_frame(frame, &app);
        }) {
            Err(error) => panic!("Error: {error:?}"),
            Ok(_) => (),
        };
        read_key(&mut app)?;
    }
    return Ok(());
}
