Rusty
=====

What is Rusty?
--------------
Rusty is a command line shell written in the Rust Language. Due to the nature
of the language used it offers fast execution and memory safety for the shell
itself (no guarantees about the programs you run with it.)

At this point in time the program is far from being functional in an every day
environment.

###The Current Version is 0.0.2 meaning Rusty can:

-[x] Execute programs entered into the command line

###Planned but not implemented:

-[ ] Shell logic
-[ ] Pipes
-[ ] Prompts and customization
-[ ] Change Directory
-[ ] Output errors on stderr automagically
-[ ] Auto Completion

###Rust Version
Currently using 1.2 Nightly due to need of unstable features.

###Testing and Execution
You can run the inbuilt tests to determine if the functions are working as
intended with the command 

'''
cargo test
'''

To test for memory leaks of the program install valgrind and run

'''
cargo build
valgrind target/debug/rusty
'''

By not executing programs and just pressing enter a few times you'll loop
through Rusty's code to test it for memory leaks. Then type exit. If other
programs are executed they can introduce their errors into the test.

You can run the program with

'''
cargo run
'''

If you want an executable to run on a daily basis (not reccomended currently)
then run the following:

'''
cargo build --release
cp target/release/rusty /destination/in/PATH
chsh user /path/to/rusty/executable
'''

###Contact
Email: mgattozzi@gmail.com
Twitter:  [@mgattozzi](https://twitter.com/mgattozzi)
