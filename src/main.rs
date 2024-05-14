#![feature(linked_list_retain)]


use std::io::{stdin, stdout, Write};
use std::io;

use game::WORDS;
use termion::{self, cursor};
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::IntoAlternateScreen;
use termion::raw::IntoRawMode;

use crate::game::GameState;
use ui::UI;

mod game;
mod ui;


macro_rules! color {
    [$c: expr] => {
        match $c {
        0 => b'g',
        1 => b'y',
        2 => b'b',
        _ => b' '
        }
    }
}


fn main() -> io::Result<()> {
    let mut out = stdout()
        .into_raw_mode()
        .expect("Error entering raw mode");
        //.into_alternate_screen()
        //.expect("Error entering alternate buffer");
    write!(&mut out, "{}", cursor::Hide)?;
    let mut ui = UI::new(&mut out);
    let mut state = GameState::new();
    let mut keys = stdin().keys();
    let mut last_guess: String = "salet".into();

    let mut buffer = [b' '; 5];
    'main : loop {
        let guess = state.get_guess(&buffer, &last_guess);
        last_guess = guess.clone();
        state = state.recalc();
        buffer = [b' '; 5];
        let mut i = 0;
        let mut selected = 0;

        ui.draw(guess.as_bytes(), &buffer, state.pos_ans()).unwrap();
        ui.stdout.flush()?;
        while let Some(Ok(c)) = keys.next() {
            match c {
                Key::Left       | Key::Char('h') if selected > 0 => {
                    ui.draw_color(color![selected], false)?;
                    selected -= 1;
                    ui.draw_color(color![selected], true)?;
                },

                Key::Right      | Key::Char('l') if selected < 2 => {
                    ui.draw_color(color![selected], false)?;
                    selected += 1;
                    ui.draw_color(color![selected], true)?;
                },

                Key::Char(' ')  | Key::Char('\n') => if i != 5 {
                    buffer[i] = color![selected];
                    ui.draw_guess(guess.as_bytes(), &buffer)?;
                    i += 1;
                } else { break; },

                Key::Backspace if i > 0 => {
                    buffer[i - 1] = b' ';
                    ui.draw_guess(guess.as_bytes(), &buffer)?;
                    i -= 1;
                },

                Key::Char('q') | Key::Esc => break 'main,

                _ => ()
            }

            ui.stdout.flush()?;
        }
    }

    ui.stdout.suspend_raw_mode()?;
    write!(ui.stdout, "{}", cursor::Show)?;
    Ok(())
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
            FilterNode::None(b) => word.contains(*b as char),
            FilterNode::NotN(b, i) => {
                let c = *b as char;
                word.as_bytes()[*i] == *b || !word.contains(c)
            },
            FilterNode::Here(b, i) => word.as_bytes()[*i] != *b
        };
    }
}































