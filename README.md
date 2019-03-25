# Rush - The Rust Shell
[![Build Status](https://travis-ci.org/joachimschmidt557/rush.svg?branch=master)](https://travis-ci.org/joachimschmidt557/rush)

**Warning:** Rush is still in development and is not feature complete or stable.

### News
Rush is not under active development.  I may continue work on it at some point but there aren't enough hours in the day.


### Features
- [x] Single command execution
- [x] Persistent history
- [x] Pipes
- [x] Quote parsing
- [x] Evironment variables
- [x] Script based config
- [x] File name completion

### Planned Features
- [ ] File redirection (partly done)
- [ ] Job control commands (fg, bg, etc.)
- [ ] Full POSIX support
- [ ] Full command completion
- [ ] Command colorization

### Posible Features
- [ ] Windows and Mac support

### Usage
- Built on rust nightly-2017-02-21
- Clone this repo 
- Build using cargo
- Copy config/rushrc.sh to ~/.rushrc

### Inspiration
Rush was orininally a fork of [Rusty](https://github.com/mgattozzi/Rusty) although I've changed a lot since then.
My peg grammar was largly taken from [js-shell-parse](https://github.com/grncdr/js-shell-parse).

### Contributing
If you'd like to contribute to the project please submit a pull request.  Help is very appreciated.
