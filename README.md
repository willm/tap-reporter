# 🚰 Tap Reporter 🚰

Tap reporter is a zero dependency [TAP](http://testanything.org/) formatter written in rust.

## Why yet another tap formatter?

Many tap formatters already exist, so why write another one ?

### Reduced maintenance burden

In the javascript eco system, most (if not all) of the existing tap formatters contain external dependencies. The TAP spec is small and doesn't change often so once an author has written it, they have little motivation to update it. Dependencies however do move on and require maintenance to keep up to date, sometimes introduce security vulnerabilities and increased maintenance. 

### To have more control over test output

It is quite common for developers to log to stdout and stderr during a test as a cheap way of logging what's going on. Support for this in existing tap reporters varies widely. Tap Reporter attempts to display this information in a useful way

### To learn rust 🦀

Rust is most likely the future of programming in a lot of ways. This project was a good opportunity to start developing some skills in it.

## How do I use it ?

1. Clone the repo, run `cargo build --release`
2. Run your tap producing test suite and pipe the output to this program `tape test.js | ./tap-reporter/target/release/tap-reporter`

## Is this project ready for the prime time

Absolutely not, it's just a work in progress at the moment
