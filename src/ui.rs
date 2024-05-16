use std::fmt::format;
use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Borders, List, ListState, Widget};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!("This is a tui template.\n\
                Press `Esc` to stop running.\n\
                Word: {}\n\
                Strikes: {}", app.word, app.strikes)
        )
        .block(
            Block::bordered()
                .title("Template")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.size(),
    );


    /*Crab emojis displayed for game board*/
    let mut displayed_game = Vec::new();
    for letter in app.game.iter() {
        displayed_game.push(Span::from(letter.clone()));
    };

    let letters = Text::from(Line::from(displayed_game));

    frame.render_widget(Paragraph::new(letters)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded)), sections[2]);


    /*Alphabet displayed for guesses*/
    let mut displayed_guesses = Vec::new();
    for guess in app.guesses.iter() {
        displayed_guesses.push(Span::from(guess.clone().red().bold()));
    };

    let guesses = Text::from(Line::from(displayed_guesses));

    frame.render_widget(Paragraph::new(guesses)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded)), sections[3]);

    match app.game_result {
        Some(true) => {
            //TODO: Make a popup window for game result
            frame.render_widget(Paragraph::new("You Win!")
                                    .centered()
                                    .block(Block::bordered()
                                        .border_type(BorderType::Rounded)), sections[1]);
        }
        Some(false) => {
            //TODO: Make a losing popout
        }
        _ => {}
    }
}
