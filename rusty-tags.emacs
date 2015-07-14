
/home/michael/Code/Rust/rusty/src/error.rs,46
pub enum InterpretError {InterpretError5,48

/home/michael/Code/Rust/rusty/src/lib.rs,80
pub mod utils;utils8,165
pub mod core;core9,180
pub mod error;error10,194

/home/michael/Code/Rust/rusty/src/utils/cd.rs,232
pub fn change_directory(input: Vec<&str>){change_directory6,212
mod tests {tests46,1420
    fn test_change_directory_ok(){test_change_directory_ok53,1530
    fn test_change_directory_fail(){test_change_directory_fail63,1816

/home/michael/Code/Rust/rusty/src/utils/calc.rs,289
pub fn calculate() {calculate4,44
fn add(num1: i64, num2: i64) -> i64 {add10,269
fn mult(num1: i64, num2: i64) -> i64 {mult14,326
fn div(num1: i64, num2: i64) -> i64 {div18,384
fn sub(num1: i64, num2: i64) -> i64 {sub22,441
fn modulo(num1: i64, num2: i64) -> i64 {modulo26,498

/home/michael/Code/Rust/rusty/src/utils/cat.rs,28
pub fn concat(){concat1,0

/home/michael/Code/Rust/rusty/src/utils/mod.rs,65
pub mod cd;cd1,0
pub mod calc;calc2,12
pub mod cat;cat3,26

/home/michael/Code/Rust/rusty/src/main.rs,23
fn main() {main9,222

/home/michael/Code/Rust/rusty/src/core/autocomplete.rs,0

/home/michael/Code/Rust/rusty/src/core/config.rs,379
fn read_in_config() -> String{read_in_config8,161
pub fn read_config_prompt(input: &Prompt) -> String {read_config_prompt26,917
pub fn check_alias(input: Vec<&str>) -> Option<String> {check_alias66,2389
pub fn set_env_var() {set_env_var94,3269
fn env_parse(input: String) -> String {env_parse109,3964
mod tests{tests148,5188
    fn readconfig() {readconfig152,5230

/home/michael/Code/Rust/rusty/src/core/script.rs,0

/home/michael/Code/Rust/rusty/src/core/execute.rs,883
pub fn interpret(command: Vec<&str>) -> String {interpret8,131
fn execute(command: Vec<&str>) -> Option<Output>{execute34,884
fn get_stdout(output: Option<Output>) -> String{get_stdout46,1247
fn get_stderr(output: Option<Output>) -> String{get_stderr59,1677
fn get_status(output: Option<Output>) -> bool{get_status70,2008
fn split_pipes(input: Vec<&str>) -> Vec<Vec<&str>> {split_pipes80,2227
fn piped(input: Vec<&str>) -> String {piped105,2933
fn first_pipe(command: Vec<&str>) -> Result<Child> {first_pipe131,3719
fn execute_pipe(command: Vec<&str>, child: Child) -> Result<Child> {execute_pipe148,4184
fn final_pipe(command: Vec<&str>, child: Child) -> String {final_pipe172,4975
mod tests{tests202,5944
    fn pipes() {pipes206,5986
    fn pipes_fail() {pipes_fail215,6218
    fn execute(){execute223,6450
    fn execute_fail(){execute_fail232,6642

/home/michael/Code/Rust/rusty/src/core/logic.rs,0

/home/michael/Code/Rust/rusty/src/core/prompt.rs,435
pub struct Prompt {Prompt7,298
impl Prompt {Prompt14,360
    pub fn new() -> Prompt {new16,375
    pub fn update_prompt(&mut self) {update_prompt23,524
    pub fn get_user_p(&self) -> String {get_user_p27,618
    pub fn get_cwd(&self) -> String {get_cwd31,697
    pub fn update_cwd(&mut self){update_cwd35,770
mod tests{tests57,1490
    fn prompt_init() {prompt_init62,1574
    fn updated_cwd() {updated_cwd69,1775

/home/michael/Code/Rust/rusty/src/core/mod.rs,179
pub mod execute;execute1,0
pub mod script;script2,17
pub mod autocomplete;autocomplete3,33
pub mod logic;logic4,55
pub mod prompt;prompt5,70
pub mod config;config6,86
/home/michael/.rusty-tags/cache/term-0.2.10.emacs,include
/home/michael/.rusty-tags/cache/toml-0.1.21.emacs,include
