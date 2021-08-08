# Multi-threaded pi calculations with Rust

Program that calculates Pi using [Ramanujan–Sato series](https://en.wikipedia.org/wiki/Ramanujan%E2%80%93Sato_series). It is written in the Rust programming language. 

Medium article link: [here](https://medium.com/@lyubo.karev/calculating-30000-pi-digits-in-10-seconds-using-multi-threaded-programming-cc417d00a217)

How to run (Ubuntu-based systems):

1. Install Rust - [explained](https://www.rust-lang.org/tools/install)
2. Clone this repo - `git clone https://github.com/lyubolp/multithreaded-pi-calculations-rust`
3. Go in the `multithreaded-pi-calculations-rust` directory
4. Install `gcc`, `make` and `m4` (`make` and `m4` are required to build the `rug` library) - `sudo apt install gcc make m4`
5. Build the executable (it will download all the needed libraries) using the command `cargo build --release` (Note: This will take several minutes)
6. Run using `cargo run` - by default, 10 000 elements will be calculated, on 2 threads. This can be changed using the command line arguments

The program could also be ran from the executable (`multithreaded-calculations-rust`, located under `./target/release`, with the following options

- The `-e` options specifies how much elements to be calculated
- The -`t` options specifies how much threads to be started

Example: `./target/release/multithreaded_pi_rust -e 10000 -t 4` - this will calculate 10 000 elements, using 4 threads.

