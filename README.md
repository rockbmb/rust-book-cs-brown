# Sketchbook for Rust Programming Language Book

This repository contains sketches of code I used while following along a
modified version of the [Rust Programming Language Book](https://rust-book.cs.brown.edu/experiment-intro.html).

This version contains an interactive component made of quizzes to aid in practicing the concepts in the book.

## Structure of the repository

Each chapter, and most of the time each subchapter of the book, will have its own folder,
appropriately identified; its contents are the result of me following the book along.

Some chapters, like 2, 12, and 20, leave to the reader the task of implementing small
practical projects, which I will try to do in their entirety.

### `minigrep` ([Chapter 12](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html))

Chapter 12 presents an extremely simple project for a `grep` that does not recurse directories,
and only searches the file specified as argument.

* Run `cargo test` to verify the executable respects the case-insensitiveness flag
* Run `cargo run -- to poem.txt` to test it on the book's example file
* Run `cargo run 2> error.txt` to see that the executable prints errors to STDERR correctly

### Mini Rust Web Server ([Chapter 20](https://rust-book.cs.brown.edu/ch20-00-final-project-a-web-server.html))

In Chapter 20, a very simple web server running on `localhost` is built. It is capable of serving
concurrent requests, using a thread pool. The Rust `async` book does the same with `async/await`.

The below `cargo` commands assume `pwd` is that of chapter 20's project.

#### Running the server

* Run `cargo run` to start the server, and then:
    - Access `127.0.0.1:7878` in a browser for the regular HTML being served
    - Access `127.0.0.1:7878/{anything}` for the HTML served in case of error
    - Access `127.0.0.1:7878/sleep` for a page equal to the first, but only served after a 5 second delay

To check the server thread pool's concurrent behavior, duplicate a `127.0.0.1:7878/sleep` tab while checking
`STDOUT`.

#### Tests and documentation

* Run `cargo test` to test the concurrent behavior of `ThreadPool`. There are currently two integration tests,
  that combine creating some files and `std::thread::sleep` to check `crate::ThreadPool` behaves correctly.

* Run `cargo doc --open` to read the package's documentation, with notes reflecting the content in chapter 20,
  and some of the author's own.