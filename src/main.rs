use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::*};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        match crossterm::event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame) {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    frame.render_widget("REDDITUI", outer_layout[0]);
    frame.render_widget("IS AWESOME", outer_layout[1]);
}
