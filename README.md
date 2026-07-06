This is the repo in which I learn how to build a shell with a series of tests as part of a guided project by Code Crafters.

> This project is done to deepen my understanding in basic system design and rust.

In this project I was able to learn:
- How a shell reads, parses, and runs commands
- What a REPL is and how it works under the hood
- How commands like echo run, vs. external ones
- What $PATH means and its role in identifying executable files
- How running commands require spawning OS processes and exits
- How to parse complex syntax
- Process Management
- How to maintain and refactor the project as it grows.

[![progress-banner](https://backend.codecrafters.io/progress/shell/046b78d7-b831-406a-a653-31ae749eb450)](https://app.codecrafters.io/users/hegde-atri?r=2qF)

This is a starting point for Rust solutions to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

In this challenge, you'll build your own POSIX compliant shell that's capable of
interpreting shell commands, running external programs and builtin commands like
cd, pwd, echo and more. Along the way, you'll learn about shell command parsing,
REPLs, builtin commands, and more.

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

# Passing the first stage

The entry point for your `shell` implementation is in `src/main.rs`. Study and
uncomment the relevant code, then run the command below to execute the tests on
our servers:

```sh
codecrafters submit
```

Time to move on to the next stage!

# Stage 2 & beyond

Note: This section is for stages 2 and beyond.

1. Ensure you have `cargo (1.96)` installed locally
1. Run `./your_program.sh` to run your program, which is implemented in
   `src/main.rs`. This command compiles your Rust project, so it might be slow
   the first time you run it. Subsequent runs will be fast.
1. Run `codecrafters submit` to submit your solution to CodeCrafters. Test
   output will be streamed to your terminal.
