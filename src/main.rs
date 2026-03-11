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
        Paragraph::new("ReddiTUI, by Kyle")
            .block(Block::new().bold().fg(Color::Red).borders(Borders::ALL)),
        outer_layout[0],
    );

    frame.render_widget(
        Paragraph::new("[content goes here]")
            .block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        outer_layout[1],
    );
}

mod reddit {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct Listing {
        data: ListingData,
    }

    #[derive(Deserialize, Debug)]
    struct ListingData {
        children: Vec<ListingChild>,
    }

    #[derive(Deserialize, Debug)]
    struct ListingChild {
        data: Post,
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct Post {
        pub title: String,
        pub selftext: String,
    }

    pub fn fetch_reddit_content() -> Vec<Post> {
        let url = "https://www.reddit.com/r/rust.json";
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0; rv:148.0")
            .build()
            .expect("failed to build");

        let resp: Listing = client
            .get(url)
            .send()
            .expect("failed to get r/rust")
            .json()
            .expect("failed to parse response");

        resp.data
            .children
            .iter()
            .map(|child| child.data.clone())
            .collect()
    }
}
