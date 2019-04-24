use std::collections::HashMap;
use prompt::Prompt;
use builtins::Builtin;

pub struct ShellState {
    pub prompt: Prompt,
    pub input_buffer: String,
    pub builtins: HashMap<String, Builtin>,
}

impl ShellState {

}
