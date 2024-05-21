use ratatui::{layout::Alignment, style::{Color, Style, Stylize as OtherStylize}, widgets::{*}, Frame, text};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::{Span, Text};
use tui_big_text::{BigText, PixelSize};
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let game_area = Layout::horizontal([
        Constraint::Min(16),
        Constraint::Min(16),
        Constraint::Min(16),
        Constraint::Min(16),
        Constraint::Min(16),
        Constraint::Min(16),
        Constraint::Min(32),
    ])
        .split(sections[1]);

    //Render widget for whole background. Make it black, give everything a border
    frame.render_widget(Block::bordered()
        .title(" Rat Trap ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .style(Style::default()
            .fg(Color::Cyan)
            .bg(Color::Black)), frame.size(),
    );

    //Render game instructions
    render_game_info(app.strikes, sections[0], frame);

    //Render game letters
    render_letters(&app.game, sections[3], frame);

    //Render guesses
    render_letters(&app.guesses, sections[4], frame);

    //Conditionally render hint
    if app.hint_display {
        render_hint(&*app.hint, sections[2], frame);
    }

    //Render trap
    draw_trap(game_area[6], frame);

    //Render rat
    match app.strikes {
        0 => draw_rat(game_area[0], frame),
        1 => draw_rat(game_area[1], frame),
        2 => draw_rat(game_area[2], frame),
        3 => draw_rat(game_area[3], frame),
        4 => draw_rat(game_area[4], frame),
        5 => draw_rat(game_area[5], frame),
        _ => draw_rat(game_area[5], frame),
    }

    //Popup for the game result
    //This need to render last to cover everything in the background
    render_game_result(app.game_result, &app.word, frame);
}

//Helper function for popup
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

/*------------------------------ Frame Rendering Functions ------------------------------*/
fn render_game_info(strikes: u8, r: Rect, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new(format!("`Enter` for hint | `Esc` to quit\n\
                Guess incorrectly 6 times and you lose!\n\
                Incorrect guesses: {}", strikes)
        )
            .block(
                Block::bordered()
                    .title(" Rat Trap ")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .centered(),
        r,
    );
}

fn render_letters(letters: &Vec<String>, r: Rect, frame: &mut Frame) {
    let mut displayed_guesses = Vec::new();
    for letter in letters.iter() {
        displayed_guesses.push(Span::from(letter.clone().red().bold()));
    };

    let guesses = Text::from(text::Line::from(displayed_guesses));

    frame.render_widget(Paragraph::new(guesses)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded))
                            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                            .centered(), r);
}

fn render_hint(hint: &str, r: Rect, frame: &mut Frame) {
    frame.render_widget(Paragraph::new("Hint: ".to_owned() + hint)
                            .centered()
                            .block(Block::bordered()
                                .border_type(BorderType::Rounded))
                            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                            .centered(), r);
}

fn render_game_result(game_result: Option<bool>, word: &String, frame: &mut Frame) {
    //TODO: Implement "play again" feature
    let area = centered_rect(60, 60, frame.size());

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Min(5),
            Constraint::Min(3),
        ])
        .split(area);

    match game_result {
        Some(true) => {
            let win_text = BigText::builder()
                .alignment(Alignment::Center)
                .pixel_size(PixelSize::Quadrant)
                .lines(vec![
                    "You Win!".cyan().into(),
                ])
                .build().unwrap();

            frame.render_widget(Clear, area); // clears out the background
            frame.render_widget(Block::bordered()
                                    .border_type(BorderType::Rounded)
                                    .style(Style::default()
                                        .fg(Color::Cyan)
                                        .bg(Color::Black)), area);
            frame.render_widget(win_text, sections[1]);
            frame.render_widget(Paragraph::new("Press `Esc` to quit")
                                    .centered(), sections[2]);
        }
        Some(false) => {
            let lose_text = BigText::builder()
                .alignment(Alignment::Center)
                .pixel_size(PixelSize::Quadrant)
                .lines(vec![
                    "You Lose!".cyan().into(),
                ])
                .build().unwrap();

            frame.render_widget(Clear, area); // clears out the background
            frame.render_widget(Block::bordered()
                                    .border_type(BorderType::Rounded)
                                    .style(Style::default()
                                        .fg(Color::Cyan)
                                        .bg(Color::Black)), area);
            frame.render_widget(lose_text, sections[1]);
            frame.render_widget(Paragraph::new(format!(
                "The word was '{}'\n\
                Press `Esc` to quit", word))
                                    .centered(), sections[2]);
        }
        _ => {}
    }
}

fn draw_rat(r: Rect, frame: &mut Frame) {
    let rat = vec![
        "      _____()()" .into(),
        "     /       @@" .into(),
        "~~~~~|_;m__m._>o".into(),
    ];

    frame.render_widget(Paragraph::new(rat)
                            .centered()
                            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                            .centered(), r);
}

fn draw_trap(r: Rect, frame: &mut Frame) {
    let trap = vec![
        "     .----------.-----------.".into(),
        "    / ____    ;..;=====.   //".into(),
        "   / /|o  |  ((()    //   // ".into(),
        "  / /o|  o| ((()    //   //  ".into(),
        " / /o_|_o_|((()====='   //   ".into(),
        "/___________'__________//    ".into(),
        "`----------'----------'`     ".into(),
    ];

    frame.render_widget(Paragraph::new(trap)
                            .centered()
                            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
                            .centered(), r);
}
