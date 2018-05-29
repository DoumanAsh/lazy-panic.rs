lazy-panic.rs
====================

[![Build Status](https://travis-ci.org/DoumanAsh/lazy-panic.rs.svg?branch=master)](https://travis-ci.org/DoumanAsh/lazy-panic.rs)
[![Crates.io](https://img.shields.io/crates/v/lazy-panic.svg)](https://crates.io/crates/lazy-panic)
[![Documentation](https://docs.rs/lazy-panic/badge.svg)](https://docs.rs/crate/lazy-panic)

Provides lazy utilities to lazily set custom panic hook

## Example

Setup simple panic message

```rust
#[macro_import]
extern crate lazy_panic;

fn main() {
    set_panic_message!(lazy_panic::formatter::Simple);

    //prints `Panic: main.rs:8 - LOLKA\n`
    painic!("LOLKA");

    set_panic_message!(lazy_panic::formatter::Debug);
    //prints `{Backtrace}\nPanic: main.rs:12 - LOLKA\n`
    painic!("LOLKA");
}
