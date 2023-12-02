# Solutions to Advent of Code 2023

This year I wanted to solve AoC using Rust 🦀 for the [second year in
a row][aoc-2022]. I didn't get the change to work
too much with Rust during the year, so I found myself quite _rusty_.
Keeping record of what I've learned last year is proving to be super useful.

I hope I can learn more stuff this year, and obviously to have the time to have
fun with AoC 2023!


## How to run the solutions

To run my solutions to AoC 2023 you need the Rust compiler. Since I decided not
to use any crate and stick with the standard libraries, that would be enough.
But I actually used `cargo` every day of the challenge to get used to the tool.
With it you can build, run and test each solution.
Check the
[Installation](https://doc.rust-lang.org/book/ch01-01-installation.html)
section in the aforementioned book.

For example, if you want to run the solution to day one, clone this repo,
and navigate to the `day-01` folder:

```
git clone https://www.github.com/santisoler/adventofocode-2023
cd adventofcode-2023
cd day-01
```

In there you can use `cargo` to test the code (if there are tests available):

```
cargo test
```

Or run the code to obtain solutions for both days:

```
cargo run
```

## License

Copyright © 2023 Santiago Soler

Source code available through the [MIT License](LICENSE).


[aoc-2022]: https://github.com/santisoler/adventofcode-2022
