#![feature(linked_list_retain)]


use std::{collections::{HashMap, LinkedList}, io::Write};
use std::io;


use crate::game::GameState;

mod game;




fn main() {
    let mut state = GameState::new();
    println!("Welcome to the Wordle Solver!");

    let mut buffer = String::with_capacity(5);
    'main : loop {
        let guess = state.get_guess(buffer.trim().as_bytes());
        buffer.clear();
        println!("\nGuess: {}", guess);
        loop {
            print!("Please enter results (b: gray, y: yellow, g: green, q: exit) : ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read input");
            buffer = buffer.trim().into();
            println!("{buffer}");

            if buffer.starts_with('q') { break 'main; }
            if buffer.trim().len() == 5 { break; } 
            println!("Please enter 5 characters");
        }

    }
}



#[derive(Copy, Clone)]
pub enum FilterNode {
    None(u8),
    NotN(u8, usize),
    Here(u8, usize)
}

impl FilterNode {
    pub fn filter(&self, word: &str) -> bool {
        return match self {
            FilterNode::None(b) => !word.contains(*b as char),
            FilterNode::NotN(b, i) => {
                let c = *b as char;
                word.as_bytes()[*i] != *b && word.contains(c)
            }
            FilterNode::Here(b, i) => {
                let c = *b as char;
                word.as_bytes()[*i] == *b && word.contains(*b as char)
            }
        };
    }
}































