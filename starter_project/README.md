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

## Rust Lessons Learned

Use `cargo watch -x check` if you're working without IDE integration (which I was);
it's not as good as in-editor feedback but it's not bad.

Use `RUST_BACKTRACE=1` to get a pretty decent backtrace on panics.

Primitive data probably should implement `Copy` and `Clone`. In my case I was a little
surprised that I couldn't grab the kind from the output of `self.peek()` since it is
a bare enum (no pointers), but you have to tell Rust that it's okay to copy the value;
I thought maybe it would just know that a bare enum is trivial to copy but it appears
the answer is no, you have to be explicit.

Recursive data always needs some kind of boxing; Rust immediately flagged my Expression
type as illegal when I had unboxed Expressions inside. This wasn't a surprise, but it
was actually nice to see how clear the error was.

## Parser Lessons Learned

**Use bottoms-up development**

I haven't made many parsers. It wasn't super obvious to me whether you want to develop
top-down or bottoms-up - when you figure out the *grammar* you pretty much have to think
top-down.

But for implementation, bottoms-up is definitely the way to go. I should have started with
nothing but numbers, then add parentheses, and gone from there. In hindsight this is obvious,
since the parser is recursive descent and that means you can only test it if everything *underneath*
the bit you are working on is already done.

Often, I think the rules I named "parenthesized" and "number" would be flattened into a single
bottom-level rule named "primary". That's often where you want to start with expressions, and
you want to get at least a subset of expressions done before you try to deal with statements
in a language that has both.


## General Lessons Learned

I accidentally had one of my `let tk = self.peek().kind` lines in the wrong place (above
the `while !finished` in the `term` function.

This predictably led to an out-of-bounds lookup in `peek()`, which paniced when I tried to
unwrap it.

The problem is, the error is a delayed side-effect of an illegal `consume` - the invariant
I wanted was that `consume` is never called except right after `peek`. Adding a panic
directly in `consume` if I detected out of bounds made it much easier to see where the bug
was.

That's a very general thing: if an invariant is violated, you want to fail as close to the
violation as possible so that you can report context (in particular a stack trace, but sometimes
there's other context we can add) that makes it easy to see where the bug happened.
