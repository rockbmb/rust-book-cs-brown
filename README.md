# Sketchbook for Rust Programming Language Book

This repository contains sketches of code I used while following along a
modified version of the [Rust Programming Language Book](https://rust-book.cs.brown.edu/experiment-intro.html).

This version contains an interactive component made of quizzes to aid in practicing the concepts in the book.

## Structure of the repository

Each chapter, and most of the time each subchapter of the book, will have its own folder,
appropriately identified; its contents are the result of me following the book along.

Some chapters, like 2, 12, and 20, leave to the reader the task of implementing small
practical projects, which I will try to do in their entirety.

### `minigrep` (Chapter 12)

* Run `cargo test` to verify the executable respects the case-insensitiveness flag
* Run `cargo run -- to poem.txt` to test it on the book's example file
* Run `cargo run 2> error.txt` to see that the executable prints errors to STDERR correctly