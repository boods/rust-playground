// Define the grammar:
// Sequence -> Sequence, Action
// Sequence -> Action
// Action -> Run | Jump | Shoot

use crate::{Action, JumpAction, RunAction, ShootAction};

enum InterpreterState {
    SeekingToken,
    ReadingToken,
    SeekingDelimiter,
}

pub struct Interpreter<'a> {
    buffer: std::str::Chars<'a>,
    next_token: String,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            buffer: input.chars(),
            next_token: String::new(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.buffer.next()
    }

    pub fn interpret(&mut self, output: &mut Vec<Box<dyn Action>>) {
        let mut state = InterpreterState::SeekingToken;

        while let Some(ch) = self.next_char() {
            match state {
                InterpreterState::SeekingToken => match ch {
                    ch if ch.is_whitespace() => continue,
                    ',' => panic!("Expected token start but found delimiter"),
                    ch => {
                        self.next_token.push(ch);
                        state = InterpreterState::ReadingToken;
                    }
                },
                InterpreterState::ReadingToken => match ch {
                    ch if ch.is_whitespace() => {
                        self.interpret_next_token(output);
                        state = InterpreterState::SeekingDelimiter;
                    }
                    ',' => {
                        self.interpret_next_token(output);
                        state = InterpreterState::SeekingToken;
                    }
                    ch => self.next_token.push(ch),
                },
                InterpreterState::SeekingDelimiter => match ch {
                    ch if ch.is_whitespace() => continue,
                    ',' => {
                        state = InterpreterState::SeekingToken;
                    }
                    ch => panic!("Expected delimiter but found other character {}", ch),
                },
            }
        }

        if !self.next_token.is_empty() {
            self.interpret_next_token(output)
        }
    }

    fn interpret_next_token(&mut self, output: &mut Vec<Box<dyn Action>>) {
        match self.next_token.as_str() {
            "Run" => output.push(Box::new(RunAction)),
            "Jump" => output.push(Box::new(JumpAction)),
            "Shoot" => output.push(Box::new(ShootAction)),
            _ => panic!("Unexpected token"),
        }
        self.next_token.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter_pattern() {
        let mut interpreter = Interpreter::new("Run, Jump, Shoot");

        let mut actions: Vec<Box<dyn Action>> = vec![];
        interpreter.interpret(&mut actions);
        assert_eq!(actions.len(), 3);
    }
}
