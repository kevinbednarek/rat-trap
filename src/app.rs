use std::error;
use std::mem::replace;
use rand::Rng;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub word: String,
    pub hint: String,
    pub hint_display: bool,
    pub guesses: Vec<String>,
    pub strikes: u8,
    pub game: Vec<String>, //This is to keep track of the actual guesses from the player
    correct_answers: Vec<String>,
    pub game_result: Option<bool>,
}

impl App {
    // Constructs a new instance of [`App`].
    pub fn new() -> App {
        let mut app = App {
            running: true,
            word: "".to_string(),
            hint: "".to_string(),
            hint_display: false,
            guesses: init_guesses(),
            strikes: 0,
            game: vec![],
            correct_answers: vec![],
            game_result: None,
        };

        init_word_and_hint(&mut app);

        app
    }

    // Set running to "false" to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn draw_char_guess(&mut self, c: char) {
        let letter = &c.to_string().to_ascii_uppercase();
        if self.guesses.contains(letter) {
            //Remove guess from list
            self.guesses.retain(|x| *x != c.to_string().to_ascii_uppercase());

            if self.word.to_ascii_uppercase().contains(letter) {
                let mut indices = Vec::new();

                for character in self.word.chars().enumerate() {
                    if character.1.to_ascii_uppercase() == c.to_ascii_uppercase() {
                        indices.push(character.0);
                    }
                }

                for idx in indices {
                    let got = replace(&mut self.game[idx], c.to_string().to_ascii_uppercase());
                    self.correct_answers.push(got);
                }

                if self.correct_answers.len() == self.word.len() {
                    self.game_result = Some(true);
                }
            } else {
                self.strikes += 1;

                if self.strikes == 6 {
                    self.game_result = Some(false);
                }

                //TODO: draw the rat trap / hangman dude
            }
        }
    }

    pub fn hint(&mut self) {
        self.hint_display = !self.hint_display;
    }
}

/*-------------------------------------- Helper Functions --------------------------------------*/
fn init_word_and_hint(app: &mut App) {
    let word_list = vec![
        (String::from("Rust"), String::from("The best programming language")),
        (String::from("Ratatui"), String::from("A Rust library for cooking up delicious TUIs")),
        (String::from("HashMap"), String::from("A data structure that holds key/value pairs")),
        //TODO: Add more words and hints
    ];

    let mut rng = rand::thread_rng();
    let rnd_num = rng.gen_range(0..word_list.len());

    let word_hint_pair = word_list.get(rnd_num);
    app.word = word_hint_pair.unwrap().clone().0;
    app.hint = word_hint_pair.unwrap().clone().1;


    //app.word = String::from("rust");
    //app.hint = String::from("The best programming language");

    app.game = vec![String::from("-"); app.word.len()];
}

fn init_guesses() -> Vec<String> {
    vec![
        String::from("A"),
        String::from(" "),
        String::from("B"),
        String::from(" "),
        String::from("C"),
        String::from(" "),
        String::from("D"),
        String::from(" "),
        String::from("E"),
        String::from(" "),
        String::from("F"),
        String::from(" "),
        String::from("G"),
        String::from(" "),
        String::from("H"),
        String::from(" "),
        String::from("I"),
        String::from(" "),
        String::from("J"),
        String::from(" "),
        String::from("K"),
        String::from(" "),
        String::from("L"),
        String::from(" "),
        String::from("M"),
        String::from(" "),
        String::from("N"),
        String::from(" "),
        String::from("O"),
        String::from(" "),
        String::from("P"),
        String::from(" "),
        String::from("Q"),
        String::from(" "),
        String::from("R"),
        String::from(" "),
        String::from("S"),
        String::from(" "),
        String::from("T"),
        String::from(" "),
        String::from("U"),
        String::from(" "),
        String::from("V"),
        String::from(" "),
        String::from("W"),
        String::from(" "),
        String::from("X"),
        String::from(" "),
        String::from("Y"),
        String::from(" "),
        String::from("Z"),
    ]
}
