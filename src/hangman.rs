use std::io::{stdin, stdout, Write};

#[derive(PartialEq)]
enum State {
    InGame { hp: i32, last_guess: Option<(char, bool)> },
    Win,
    Lose,
}

pub struct Game {
    word: String,
    category: String,
    pub word_encrypted: String,
    state: State,
}

impl Game {
    pub fn new(word: String, category: String) -> Self {
        Self {
            word: word.clone(),
            category,
            word_encrypted: word.chars().into_iter().map(|_| '*').collect::<String>(),
            state: State::InGame { hp: 5, last_guess: None },
        }
    }

    pub fn run(&mut self) {
        while let State::InGame { .. } = self.state {
            self.print_info();
            let mut guess = String::new();
            print!("Guess character: ");
            stdout().flush().unwrap();
            stdin().read_line(&mut guess).expect("stdin error");
            self.guess_character(guess.trim().chars().collect::<Vec<char>>()[0]);
        }
        self.print_info();
    }

    fn guess_character(&mut self, guess: char) {
        let mut result: bool = false;
        let chars: Vec<char> = self.word.chars().collect();
        self.word_encrypted = self.word_encrypted.chars().enumerate().map(
            |(i, c)| {
                if c == '*' && guess == chars[i] {
                    result = true;
                    guess
                } else { c }
            }
        ).collect();

        if !self.word_encrypted.contains("*") {
            self.state = State::Win;
            return;
        }

        if let State::InGame { mut hp, .. } = self.state {
            if result == false {
                hp -= 1;
            }

            self.state = if hp == 0 {
                State::Lose
            } else {
                State::InGame { hp, last_guess: Some((guess, result)) }
            }
        }
    }

    fn print_info(&self) {
        print!("\n\n");

        match self.state {
            State::Win => {
                println!("Congratulations, you won! Word: {}", self.word);
            }
            State::Lose => {
                println!("You lost! Word: {}", self.word);
            }
            State::InGame { hp, last_guess } => {
                println!("Word: {} - Category: {} - HP: {}", self.word_encrypted, self.category, hp);
                if let Some((guess, correct)) = last_guess {
                    if correct {
                        println!("Last guess: {}", guess);
                    } else {
                        println!("Wrong guess '{}'", guess)
                    }
                }
            }
        }
    }
}