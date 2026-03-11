use crossterm::event::{Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::*,
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Subreddit to browse
    #[arg(value_name="SUBREDDIT", default_value="rust")]
    subreddit: String,
}

struct State {
    posts: Vec<reddit::Post>,
    selected: usize,
    subreddit: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let posts = reddit::fetch_reddit_content();
    let args = Args::parse();
    let mut state = State { posts, selected: 0, subreddit: args.subreddit };
    ratatui::run(|terminal| app(terminal, &mut state))?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal, state: &mut State) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        match crossterm::event::read()? {
            Event::Key(key) => {
                if key.code == KeyCode::Char('q') {
                    break Ok(());
                }
                if key.code == KeyCode::Char('j') {
                    if state.selected != 0 {
                        state.selected -= 1;
                    }
                }
                if key.code == KeyCode::Char('k') {
                    if state.posts.get(state.selected + 1).is_some() {
                        state.selected += 1;
                    }
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, state: &State) {
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(8), Constraint::Percentage(8), Constraint::Percentage(85)])
        .split(frame.area());

    frame.render_widget(
        Paragraph::new(format!("ReddiTUI | Browsing r/{}", state.subreddit))
            .block(Block::new().bold().fg(Color::Red).borders(Borders::ALL)),
        outer_layout[0],
    );

    let content = if let Some(post) = state.posts.get(state.selected) {
        format!("{}  [{} upvotes]", post.title, post.ups)
    } else {
        "No posts loaded".to_string()
    };

    frame.render_widget(
        Paragraph::new(content).block(Block::new().bold().fg(Color::Cyan).borders(Borders::ALL)),
        outer_layout[1],
    );

    let content = if let Some(post) = state.posts.get(state.selected) {
        format!("{}", post.selftext)
    } else {
        "".to_string()
    };

    frame.render_widget(
        Paragraph::new(content).block(Block::new().bold().fg(Color::Blue).borders(Borders::ALL)),
        outer_layout[2],
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
        pub ups: u32,
    }

    pub fn fetch_reddit_content() -> Vec<Post> {
        use clap::Parser;

        let args = super::Args::parse();
        let url = format!("https://www.reddit.com/r/{}.json", args.subreddit);
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
