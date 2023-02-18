# oercalc - Overly Engineered Rust Calculator

## What is this?
My Rust starter project I decided to do for fun. A simple calculator that takes in an arbitrary number of arguments and performs a defined operation.

## Usage

### Without arguments
Just run this and it will guide you through it step-by-step.

### With command line arguments
Write `cargo run -- operation arguments`.

Operations are defined as follows: 
- `add` - addition;
- `sub` - subtraction;
- `mul` - multiplication;
- `div` - division.

Example:
```
cargo run -- add 2 2
    Compiling oercalc v0.1.0 (/Users/8ball/Development/Rust/Churns/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s
    Running `target/debug/oercalc add 2 2`
Result: 4
```

## Todo

- [] Implement TUI
- [] Implement help command.