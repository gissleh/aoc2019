# AOC 2019 Answers

This is my repo for AOC answers. My goal is to practice Rust
which is a language I do not get much practical use out of
currently.

I'll try to keep the code neat, but if a safe-ish `unsafe` 
knocks a few µs off the duration, then... 🙈.

If you want to use this as a template for your own AOC answers,
leave only `lib.rs` and `aoc.rs`. `cargo test` must be run with
`--bin dayXX` for it to work on an incomplete repo.

`dl-input.sh` downloads the puzzle inputs. Add a `AOC_COOKIE`
variable to a cookie.env file to use it.