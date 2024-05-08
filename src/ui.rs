use std::io::{self, Stdout, Write};

use termion::{cursor::Goto, raw::RawTerminal};











pub struct UI<'a> {
    pub stdout      : &'a mut RawTerminal<Stdout>,
    guess_start     : (u16, u16),
    color_start     : (u16, u16),
    loading_start   : (u16, u16),
}

impl<'a> UI<'a> {
    pub fn new(stdout: &'a mut RawTerminal<Stdout>) -> UI<'a> {
        let size = termion::terminal_size()
            .expect("womp womp");
        let mut guess_start = (0, 0);
        let mut color_start = (0, 0);
        let mut loading_start = (0, 0);

        let center = (size.0 / 2, size.1 / 2);
        color_start.0 = center.0 - 8;
        color_start.1 = center.1 + 2;

        loading_start.0 = center.0 - 11;
        loading_start.1 = center.1;

        guess_start.0 = center.0 - 7;
        guess_start.1 = center.1 - 5;


        return Self {
            stdout,
            guess_start,
            color_start,
            loading_start,
        };
    }

    pub fn draw(&mut self, guess: &[u8], colors: &[u8; 5], possible_guesses: usize) -> io::Result<()> {
        self.draw_guess(guess, colors)?;
        write!(self.stdout, "{}{}{}Remaining: {possible_guesses}", 
            Goto(self.color_start.0, self.color_start.1 - 1),
            termion::clear::CurrentLine,
            termion::color::Bg(termion::color::Reset)
        )?;
        self.draw_color(b'g', true)?;
        self.draw_color(b'y', false)?;
        self.draw_color(b'b', false)?;

        Ok(())
    }

    pub fn draw_color(&mut self, color: u8, highlighted: bool) -> io::Result<()> {
        let (c, offset) = match color {
            b'g' => (termion::color::Bg(termion::color::Rgb(108, 169, 101)).to_string(), 0),
            b'y' => (termion::color::Bg(termion::color::Rgb(200, 182, 83)).to_string(), 1),
            b'b' => (termion::color::Bg(termion::color::Rgb(120, 124, 127)).to_string(), 2),
            _ => (termion::color::Bg(termion::color::Reset).to_string(), 0)
        };

        if highlighted {
            write!(self.stdout, "{}{}╔═══╗{}║   ║{}╚═══╝", 
                Goto(self.color_start.0 + offset * 6, self.color_start.1),
                c,
                Goto(self.color_start.0 + offset * 6, self.color_start.1 + 1),
                Goto(self.color_start.0 + offset * 6, self.color_start.1 + 2),
            )?;
        } else {
            write!(self.stdout, "{}{}┌───┐{}│   │{}└───┘", 
                Goto(self.color_start.0 + offset * 6, self.color_start.1),
                c,
                Goto(self.color_start.0 + offset * 6, self.color_start.1 + 1),
                Goto(self.color_start.0 + offset * 6, self.color_start.1 + 2),
            )?;
        }

        Ok(())
    }

    pub fn draw_guess(&mut self, guess: &[u8], colors: &[u8; 5]) -> io::Result<()> {
        for i in 0..5 {
            let c = match colors[i] {
                b'g' => termion::color::Bg(termion::color::Rgb(108, 169, 101)).to_string(),
                b'y' => termion::color::Bg(termion::color::Rgb(200, 182, 83)).to_string(),
                b'b' => termion::color::Bg(termion::color::Rgb(120, 124, 127)).to_string(),
                _ => termion::color::Bg(termion::color::Reset).to_string()
            };

            write!(self.stdout, "{}{}┌─┐{}│{}│{}└─┘{}{} {}", 
                Goto(self.guess_start.0 + i as u16 * 3, self.guess_start.1),
                c,
                Goto(self.guess_start.0 + i as u16 * 3, self.guess_start.1 + 1),
                (guess[i] as char).to_uppercase(),
                Goto(self.guess_start.0 + i as u16 * 3, self.guess_start.1 + 2),
                termion::color::Bg(termion::color::Reset),
                Goto(self.guess_start.0 + i as u16 * 3, self.guess_start.1 + 3),
                (colors[i] as char).to_uppercase()
            )?;
        }

        Ok(())
    }

    /*
    * progress should be between 0 - 10
    */
    pub fn draw_loading_bar(&mut self, progress: usize) -> io::Result<()> {
        /*
        let mut bar = "██".repeat(progress);
        bar.push_str(&"·".repeat(20 - progress * 2));

        write!(self.stdout, "{}{}[{}]",
            Goto(self.loading_start.0, self.loading_start.1),
            termion::color::Bg(termion::color::Reset),
            bar
        )?;
*/

        Ok(())
    }
}






























