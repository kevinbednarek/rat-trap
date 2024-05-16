use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{canvas::*, *},
    Frame,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Line, Span, Text};
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
        Paragraph::new(format!("Press `Esc` to quit\n\
                Press `Enter` for a hint\n\
                Word: {}\n\
                Strikes: {}", app.word, app.strikes)
        )
        .block(
            Block::bordered()
                .title(" Rat Trap ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .centered(),
        frame.size(),
    );

    /*--- Hyphens displayed for the letters of the word to guess ---*/
    let mut displayed_game = Vec::new();
    for letter in app.game.iter() {
        displayed_game.push(Span::from(letter.clone()));
    };

    let letters = Text::from(Line::from(displayed_game));

    frame.render_widget(Paragraph::new(letters)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded)), sections[2]);


    /*--- Alphabet displayed for guesses ---*/
    let mut displayed_guesses = Vec::new();
    for guess in app.guesses.iter() {
        displayed_guesses.push(Span::from(guess.clone().red().bold()));
    };

    let guesses = Text::from(Line::from(displayed_guesses));

    frame.render_widget(Paragraph::new(guesses)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded)), sections[3]);

    /*--- Conditionally display a hint ---*/
    if app.hint_display {
        frame.render_widget(Paragraph::new("Hint: ".to_owned() + &*app.hint)
                                .centered()
                                .block(Block::bordered()
                                    .border_type(BorderType::Rounded)), sections[1]);
    }

    /*--- Popup for the game result ---*/
    match app.game_result {
        Some(true) => {
            let area = centered_rect(60, 30, frame.size());
            //frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(Paragraph::new(
                "You Win!\n\
                Press `Esc` to quit")  //TODO: Implement "play again" feature
                                    .centered()
                                    .block(Block::bordered()
                                        .border_type(BorderType::Rounded)), area);
        }
        Some(false) => {
            let area = centered_rect(60, 30, frame.size());
            //frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(Paragraph::new(format!(
                "You Lose!\n\
                The word was '{}'\n\
                Press `Esc` to quit", app.word))  //TODO: Implement "play again" feature
                                    .centered()
                                    .block(Block::bordered()
                                        .border_type(BorderType::Rounded)), area);
        }
        _ => {}
    }
}

// Helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
        .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
        .split(popup_layout[1])[1]
}
