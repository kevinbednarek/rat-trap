use std::error;
use std::mem::replace;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub word: String,
    pub hint: String,
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
            guesses: init_guesses(),
            strikes: 0,
            game: vec![],
            correct_answers: vec![],
            game_result: None,
        };

        init_word(&mut app);

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
                    if character.1 == c {
                        indices.push(character.0);
                    }
                }

                for idx in indices {
                    let got = replace(&mut self.game[idx], c.to_string().to_ascii_uppercase());
                    self.correct_answers.push(got);
                }

                if self.correct_answers.len() == self.word.len() {
                    //TODO: Do a winning sequence
                    you_win();
                    self.game_result = Some(true);
                }
            } else {
                self.strikes += 1;

                if self.strikes == 6 {
                    //TODO: call method for "you lost" sequence
                    you_lose();
                    self.game_result = Some(false);
                }

                //TODO: draw the rat trap / hangman dude
            }
        }
    }

    pub fn hint(&mut self) {
        self.hint = String::from("The best programming language");
    }
}

/*-------------------------------------- Helper Functions --------------------------------------*/
pub fn init_word(app: &mut App) {
    app.word = String::from("rust");
    //TODO: create list of words
    //TODO: pick a random number, use the number as the index and return the word that corresponds to the index

    //app.game = vec![emojis::get("🦀").unwrap().to_string(); app.word.len()];
    app.game = vec![String::from("-"); app.word.len()];
}

pub fn you_lose() {
    //TODO: Draw the last piece of the rat trap / hangman guy

    //TODO: Do a popup showing answer and asking if you want to play again
}

pub fn you_win() {
    //TODO: Draw some animation?

    //TODO: Do a popup showing answer and asking if you want to play again
}

pub fn init_guesses() -> Vec<String> {
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
