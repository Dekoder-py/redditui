use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::*,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};

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
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Fill(1), Constraint::Fill(5)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new("ReddiTUI, by Kyle").block(Block::new().bold().fg(Color::Red).borders(Borders::ALL)),
        outer_layout[0],
    );

    frame.render_widget(
        Paragraph::new("[content goes here]").block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        outer_layout[1],
    );
}
