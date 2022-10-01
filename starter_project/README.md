# A simple getting-started proejct in Rust

I have only used Rust for a few small things (a couple toy projects and some commits
here and there to existing codebases at work).

I'd like to get to know it better since the LibCST parser is in Rust and my team at work
is considering working on an error-recovering parser; rather than going through the Rust
book again (which I should do at some point) I'd prefer to get started by building
some very simple parsing-related examples.

This project makes a command-line calculator via a hand-written lexer and parser. The
code is very closely based on an example Crafting Interpreters in Rust codebase I looked
at, https://github.com/UncleScientist/lox-ast/tree/main/src, although I wrote this from
scratch and broke things down a bit differently (plus I'm sticking to a much smaller subset
of the full `lox` grammar for now).

I'm trying to follow real patterns for a handwritten lexer and parser, but keep the project
trivial enough that I can use it for getting started with rust; jumping into anything
serious would probably be overwhelming since (as a primarily Python / Ocaml / Scala dev)
I'm not at all used to dealing with ownership, borrow checking, or lifetimes.

## Creating the project

Set up the project:
```
cargo new starter_project
```

This makes the `Cargo.toml` and `src/main.rs`.

Install cargo watch:
```
cargo install cargo-watch
```

Run a compile loop:
```
cargo watch -x check
```

Run the calculator:
```
cargo run
```

