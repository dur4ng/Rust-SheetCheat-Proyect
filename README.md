# Rust Crash Course

## Install Rust

Rust is required for this course!  The latest stable version is always recommended.

- Go to [rust-lang.org](https://rust-lang.org) and click on the `Get Started`
   button and follow the instructions to install Rust for your operating system.
   - Please DO NOT install rust via some other package manager.  It will probably be a version that is _really old_.

You should get somewhat similar output if you run commands like the ones below (newer versions are okay).  If you 
already have an old version of Rust installed, then run `rustup update` to install a newer version.

```shell
$ rustc --version
rustc 1.49.0 (e1884a8e3 2020-12-29)
$ cargo --version
cargo 1.49.0 (d00d64df9 2020-12-05)
```

- Clone or download this repository to your computer.

## Prepare Your Development Environment

Please do the following (see the [How To Learn Rust](https://github.com/CleanCut/ultimate_rust_crash_course/blob/master/HowToLearnRust.md)
page for details on all of these)
- [ ] Choose an IDE (or Editor) and configure it with Rust support and customize it to your liking
- [ ] Choose one place to "find answers" and either introduce yourself (if it's a forum, IRC, etc.) or find the answer
      to one question you have.
- [ ] Try doing something in Rust!  If you don't have a better idea, then just do this:
  - `cargo new message`
  - `cd message`
  - `cargo run`
  - Edit `src/main.rs` and change the message.
  - `cargo run` again to see your new message.
- [ ] Check out the descriptions of the tools and books.

# Training!

Now you are ready for the training!  Go watch the [Ultimate Rust Crash Course] (or attend the live
session) and come back here for the [exercises].

# Resources

- Training by the instructor (Nathan Stocks) in the form of the [Ultimate Rust Crash Course] or a
  live session.
- This Repository :tada:
- [How To Learn Rust](https://github.com/CleanCut/rust_a_crash_course/blob/master/HowToLearnRust.md)
- [The Rust Standard Library](https://doc.rust-lang.org/std/)

# Exercises

Please clone this repository! These exercises are designed as Rust projects for you to edit on your
own computer, with the exception of Exercise A (which is just a `README.md` file).

The exercises are separate Rust projects inside the `exercises/` subdirectory.  For each exercise,
you should:
- Open the corresponding`exercise/EXERCISE_NAME` directory in your IDE/Editor
  - Seriously, just open the _individual exercise directory_ in your IDE. If you open the entire repository, your IDE will probably complain that it sees multiple Rust projects.
- Navigate to the same directory with your Terminal application (so you can run `cargo run`, etc.)
- Open up the `src/main.rs` file.
- Follow the numbered exercise instructions in the code comments.
