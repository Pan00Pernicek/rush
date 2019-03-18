extern crate rustyline;

use self::rustyline::{Config, CompletionType, Editor, Helper};
use self::rustyline::completion::{Completer, FilenameCompleter, Pair};
use self::rustyline::hint::Hinter;
use self::rustyline::highlight::Highlighter;
use self::rustyline::error::ReadlineError;
use std::borrow::Cow::{self, Borrowed, Owned};

pub struct RushHelper(pub FilenameCompleter);

impl Completer for RushHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.0.complete(line, pos)
    }
}

impl Highlighter for RushHelper {
    fn highlight_prompt<'p>(&self, prompt: &'p str) -> Cow<'p, str> {
        Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
    }
}

impl Hinter for RushHelper {
    fn hint(&self, line: &str, _pos: usize) -> Option<String> {
        if line == "hello" {
            Some(" World".to_owned())
        } else {
            None
        }
    }
}

impl Helper for RushHelper {}
