use ratatui::{
    layout::Alignment,
    style::{Color, Style, Stylize},
    widgets::{canvas::*, *},
    Frame,
};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Buffer, Marker};
use ratatui::text::{Line, Span, Text};
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
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
        Paragraph::new(format!("`Enter` for hint | `Esc` to quit\n\
                Guess incorrectly 6 times and you lose!\n\
                Incorrect guesses: {}", app.strikes)
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

    match app.strikes {
        1 => {/*TODO: Draw something*/}
        2 => {/*TODO: Draw something*/}
        3 => {/*TODO: Draw something*/}
        4 => {/*TODO: Draw something*/}
        5 => {/*TODO: Draw something*/}
        6 => {/*TODO: Draw last thing*/}
        _ => {}

    }
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


/*fn draw_something(x: f64, y: f64) -> impl Widget + '_ {
    Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("World"))
        .marker(self.marker)
        .paint(|ctx| {
            ctx.draw(&Map {
                color: Color::Green,
                resolution: MapResolution::High,
            });
            ctx.print(self.x, -self.y, "You are here".yellow());
        })
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
}*/

fn draw_map()  {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 40));
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|context| {
            context.draw(&Map {
                resolution: MapResolution::High,
                ..Default::default()
            });
        });
    canvas.render(buffer.area, &mut buffer);
}


fn draw_high() {
    let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 40));
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|context| {
            context.draw(&Map {
                resolution: MapResolution::High,
                ..Default::default()
            });
        });
    canvas.render(buffer.area, &mut buffer);
    let expected = Buffer::with_lines([
        "                                                                                ",
        "                  ⢀⣠⠤⠤⠤⠔⢤⣤⡄⠤⡠⣄⠢⠂⢢⠰⣠⡄⣀⡀                      ⣀                   ",
        "            ⢀⣀⡤⣦⠲⢶⣿⣮⣿⡉⣰⢶⢏⡂        ⢀⣟⠁     ⢺⣻⢿⠏   ⠈⠉⠁ ⢀⣀    ⠈⠓⢳⣢⣂⡀               ",
        "            ⡞⣳⣿⣻⡧⣷⣿⣿⢿⢿⣧⡀⠉⠉⠙⢆      ⣰⠇               ⣠⠞⠃⢉⣄⣀⣠⠴⠊⠉⠁ ⠐⠾⠤⢤⠤⡄⠐⣻⠜⢓⠂      ",
        "⢍ ⢀⡴⠊⠙⠓⠒⠒⠤⠖⠺⠿⠽⣷⣬⢬⣾⣷⢻⣷⢲⢲⣍⠱⡀ ⠹⡗   ⢀⢐⠟        ⡔⠒⠉⠲⠤⢀⢄⡀⢩⣣⠦⢷⢼⡏⠈          ⠉⠉⠉ ⠈⠈⠉⠖⠤⠆⠒⠭",
        "⠶⢽⡲⣽⡆             ⠈⣠⣽⣯⡼⢯⣘⡯⠃⠘⡆ ⢰⠒⠁ ⢾⣚⠟    ⢀⠆ ⣔⠆ ⢷⠾⠋⠁    ⠙⠁                     ⠠⡤",
        "  ⠠⢧⣄⣀⡶⠦⠤⡀        ⢰⡁ ⠉⡻⠙⣎⡥  ⠘⠲⠇       ⢀⡀⠨⣁⡄⣸⢫⡤⠄                        ⣀⢠⣤⠊⣼⠅⠖⠋⠁",
        "   ⣠⠾⠛⠁  ⠈⣱        ⠋⠦⢤⡼ ⠈⠈⠦⡀         ⢀⣿⣇ ⢹⣷⣂⡞⠃                       ⢀⣂⡀  ⠏⣜    ",
        "          ⠙⣷⡄        ⠘⠆ ⢀⣀⡠⣗         ⠘⣻⣽⡟⠉⠈                           ⢹⡇  ⠟⠁    ",
        "           ⠈⡟           ⢎⣻⡿⠾⠇         ⠘⠇  ⣀⡀  ⣤⣤⡆ ⡠⡦                 ⢀⠎⡏        ",
        "            ⡇          ⣀⠏⠋           ⢸⠒⢃⡖⢻⢟⣷⣄⣰⣡⠥⣱ ⢏⣧              ⣀ ⡴⠚⢰⠟        ",
        "            ⢳         ⢸⠃             ⠸⣄⣼⣠⢼⡴⡟⢿⢿⣀⣄  ⠸⡹             ⠘⡯⢿⡇⡠⢼⠁        ",
        "             ⢳⣀      ⢀⠞⠁             ⢠⠋⠁ ⠐⠧⡄⣬⣉⣈⡽                  ⢧⠘⢽⠟⠉         ",
        "              ⣿⣄  ⡴⠚⠛⣿⣀             ⢠⠖     ⠈⠁ ⠹⣧  ⢾⣄⡀             ⡼ ⠈           ",
        "    ⣀         ⠘⣿⡄ ⡇  ⣘⣻             ⡏          ⢻⡄ ⠘⠿⢿⠒⠲⡀   ⢀⡀   ⢀⡰⣗             ",
        "    ⠉⠷          ⢫⡀⢧⡼⡟⠉⣛⣳⣦⡀         ⠈⡇          ⠸⣱  ⢀⡼  ⢺  ⡸⠉⢇  ⣾⡏ ⣁             ",
        "                 ⠉⠒⢆⡓⡆             ⠠⡃           ⢳⣇⡠⠏   ⠐⡄⡞  ⠘⣇⡀⢱  ⣾⡀            ",
        "                    ⢹⣇⣀⣾⡷⠤⡆         ⢣            ⠯⢺⠇    ⢣⣅   ⣽⢱⡔ ⢠⢿⣗            ",
        "                     ⠙⢱   ⠘⠦⡄       ⠈⢦⡠⣠⢶⣀        ⡜     ⠈⠿  ⢠⣽⢆ ⢀⣼⡜⠿            ",
        "                     ⢀⡞     ⢱⡀           ⢸       ⡔⠁          ⢻⢿⢰⠏⢸⣤⣴⣆           ",
        "                     ⢘⠆      ⠙⠢⢄         ⠸⡀     ⡸⠁           ⠈⣞⡎⠥⡟⣿⠠⠿⣷⠒⢤⢀⣆      ",
        "                     ⠘⠆        ⢈⠂         ⢳     ⡇             ⠈⠳⠶⣤⣭⣠ ⠋⢧⡬⣟⠉⠷⡄    ",
        "                      ⢨        ⡜          ⢸     ⠸ ⣠               ⠁⢁⣰⢶ ⡇⠉⠁ ⠛    ",
        "⠆                     ⠈⢱⡀      ⡆          ⡇    ⢀⡜⡴⢹               ⢰⠏⠁⠘⢶⠹⡀   ⠸ ⢠⡶",
        "                        ⠅     ⣸           ⢸    ⢫ ⡞⡊             ⢠⠔⠋     ⢳⡀ ⠐⣦   ",
        "                        ⡅    ⡏            ⠈⡆  ⢠⠎ ⠳⠃             ⢸        ⢳      ",
        "                       ⠨    ⡸⠁             ⢱  ⡸                 ⠈⡇ ⢀⣀⡀   ⢸      ",
        "                       ⠸  ⠐⡶⠁              ⠘⠖⠚                   ⠣⠒⠋ ⠱⣇ ⢀⠇   ⠰⡄ ",
        "                       ⠽ ⣰⡖⠁                                          ⠘⢚⡊    ⢀⣿⠇",
        "                       ⡯⢀⡟                                             ⠘⠏   ⢠⢾⠃ ",
        "                       ⠇⢨⠆                            ⢠⡄                    ⠈⠁  ",
        "                       ⢧⣷⡀⠚                                                     ",
        "                        ⠉⠁                                                      ",
        "                          ⢀⡀                                                    ",
        "                        ⢠⡾⠋                      ⣀⡠⠖⢦⣀⣀  ⣀⠤⠦⢤⠤⠶⠤⠖⠦⠤⠤⠤⠴⠤⢤⣄       ",
        "                ⢀⣤⣀ ⡀  ⣼⣻⠙⡆         ⢀⡤⠤⠤⠴⠒⠖⠒⠒⠒⠚⠉⠋⠁    ⢰⡳⠊⠁              ⠈⠉⠉⠒⠤⣤  ",
        "    ⢀⣀⣀⡴⠖⠒⠒⠚⠛⠛⠛⠒⠚⠳⠉⠉⠉⠉⢉⣉⡥⠔⠃     ⢀⣠⠤⠴⠃                                      ⢠⠞⠁  ",
        "   ⠘⠛⣓⣒⠆              ⠸⠥⣀⣤⡦⠠⣞⣭⣇⣘⠿⠆                                         ⣖⠛   ",
        "⠶⠔⠲⠤⠠⠜⢗⠤⠄                 ⠘⠉  ⠁                                            ⠈⠉⠒⠔⠤",
        "                                                                                ",
    ]);
    assert_eq!(buffer, expected);
}