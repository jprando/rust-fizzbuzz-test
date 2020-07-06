# FizzBuzz Test

Write a program that prints the numbers from 1 to 100.  
But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”.  
For numbers which are multiples of both three and five print “FizzBuzz”.

[wikipedia](https://en.wikipedia.org/wiki/Fizz_buzz)

## In this repository

The implementation of the FizzBuzz test in the [rust language](https://www.rust-lang.org/).

## fn ShowMe() { the_code }

content of [main.rs](/src/main.rs) file

```rust
fn main() {
  (1..101u8).map(|num| match(num % 3, num % 5) {
    (0, 0) => "FIZZBUZZ".into(),
    (0, _) => "FIZZ".into(),
    (_, 0) => "BUZZ".into(),
    _ => num.to_string()
    }).for_each(|num| println!("{}", num))
}
```

## Benchmark

```shell
❯ hyperfine -m 1000 target/release/fizzbuss-x86_64-linux
Benchmark #1: target/release/fizzbuss-x86_64-linux
  Time (mean ± σ):       1.8 ms ±   0.4 ms    [User: 1.2 ms, System: 0.7 ms]
  Range (min … max):     1.1 ms …   3.0 ms    1000 runs
```