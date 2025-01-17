# Background 

This is my first experience coding in Rust. To learn, I plan to work through the challenges presented in the Advent of Code to learn the fundamentals, discover best practices, and employ common coding methodologies.

The Advent of code is described as follows:

> The [Advent of Code](https://adventofcode.com/) (AoC or aoc) is an Advent calendar of small programming puzzles for a variety of skill levels that can be solved in any programming language you like. People use them as interview prep, company training, university coursework, practice problems, a speed contest, or to challenge each other.

## Inspiration

I have taken inspiration from [Christopher Biscardi
](https://github.com/ChristopherBiscardi/advent-of-code/tree/bc5ad528576194355f2fb1b6a03091d8bcd69f76/2024)'s AoC workspace design and toolkit. I will work to implement them while learning Rust's best practices.

## Setup

The [_get-aoc-input.rs_](scripts/get-aoc-input.rs) must be executed once to install the project setup dependencies. For it to successfully execute, it also requires the Rust nightly tool chain. To install all the dependencies, execute:

```sh
rustup toolchain install nightly
./scripts/get-aoc-input.rs
```

### Just

The project utilizes [just](https://github.com/casey/just?tab=readme-ov-file) as a command runner to automate project creation, testing, and verification. Ensure that the _just_ executable is on the `PATH` by executing:

```sh
which just
```

### Cargo

#### Generate

To create new Rust projects with a build template, Cargo's [generate](https://docs.rs/generate/latest/generate/) crate is utilized. It must be first installed from the project's top level (_rust/_) using Cargo:

```sh
cargo install cargo-generate
```

#### Nextest

See the [cargo-nextest](https://nexte.st/docs/installation/pre-built-binaries/#with-cargo-binstall) documentation for installing nextest on your device.

I chose to install the _binstall_ crate following the [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) docs.

```sh
cargo install cargo-binstall # Required to install nextest via binstall
cargo binstall cargo-nextest --secure
```

#### Clippy

The code is linted using [Clippy](https://github.com/rust-lang/rust-clippy). This is optional and can be installed using the the following commands:

```sh
rustup update
rustup component add clippy
cargo clippy
cargo clippy --fix
```

#### Watch

```sh
cargo binstall cargo-watch
```