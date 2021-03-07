# Advent of Code 2017 in Rust

My solutions to [Advent of Code 2017](https://adventofcode.com/2017) solved with Rust :heart:

I'm currently learning Rust, and doing Advent of Code is a great way to learn a new language.


## Running each puzzle

Each day is a separate Cargo project for now. You can `cd` into it and run tests and the binary. The puzzle answer is obtained by running the binary.

```
$ cd day_02
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running target/debug/deps/day_02-a924dbf3a6cdab6c

running 4 tests
test tests::row_checksum_division_test ... ok
test tests::row_checksum_test ... ok
test tests::spreadsheet_checksum_by_division_test ... ok
test tests::spreadsheet_checksum_test ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/day_02`
The answer to D2P1 is 32020
The answer to D2P2 is 236
```
