use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        match crossterm::event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }

                if key.code == KeyCode::Char('r') {
                    terminal.draw(rick)?;
                } else {
                    terminal.draw(render)?;
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("REDDITUI", frame.area());
}

fn rick(frame: &mut Frame) {
    frame.render_widget("Never gonna give you up!", frame.area());
}
