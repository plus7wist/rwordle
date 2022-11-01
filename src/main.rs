use anyhow::Result;
use rand::Rng;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashSet;

mod word;

#[derive(Debug)]
struct WordTable {
    common_list: Vec<&'static str>,
    common: HashSet<&'static str>,
    rare: HashSet<&'static str>,
    rng: rand::rngs::ThreadRng,
}

impl WordTable {
    fn new() -> Self {
        Self {
            common: word::common().into_iter().collect(),
            common_list: word::common(),
            rare: word::rare().into_iter().collect(),
            rng: rand::thread_rng(),
        }
    }

    fn is_word(&self, word: &str) -> bool {
        self.common.contains(word) || self.rare.contains(word)
    }

    fn random_word(&mut self) -> &str {
        let i = self.rng.gen_range(0..self.common.len());
        &self.common_list[i]
    }
}

fn main() -> Result<()> {
    let mut wt = WordTable::new();

    let mut word = wt.random_word().to_string();

    let mut tryi = 0;
    let mut trys: Vec<Option<String>> = vec![None; 5];

    let mut rl = Editor::<()>::new()?;

    loop {
        // Print all trys

        let mut win = false;

        println!();
        for each in &trys {
            match each {
                None => (),
                Some(s) => {
                    for c in s.chars() {
                        print!("{} ", c)
                    }
                    println!();

                    let hashset: HashSet<char> = word.chars().collect();

                    for (c, good_c) in s.chars().zip(word.chars()) {
                        let p = if c == good_c {
                            'y'
                        } else if hashset.contains(&c) {
                            'c'
                        } else {
                            'n'
                        };
                        print!("{} ", p);
                    }
                    println!();

                    if s == &word {
                        win = true;
                        break;
                    }
                }
            }
        }

        if win {
            println!("you win!");
            break;
        }

        // Check lose

        if tryi == trys.len() {
            println!("you lose");
            break;
        }

        let line = rl.readline(">> ");

        let guess = match line {
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Ok(command) => match command.as_str() {
                "quit" => break,
                "new" => {
                    word = wt.random_word().to_string();
                    trys = vec![None; 5];
                    continue;
                }
                "show" => {
                    println!("current word: {}", word);
                    continue;
                }
                _ => command,
            },
            Err(err) => return Err(err.into()),
        };

        let guess = guess.trim().to_ascii_uppercase();
        if guess.len() != 5 {
            println!("invalid word length");
            continue;
        }

        if !wt.is_word(&guess) {
            println!("not word");
            continue;
        }

        trys[tryi] = Some(guess.to_string());
        tryi += 1;
    }

    println!("bye");

    Ok(())
}
