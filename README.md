# FizzBuzz Test

Write a program that prints the numbers from 1 to 100.  
But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”.  
For numbers which are multiples of both three and five print “FizzBuzz”.

[wikipedia](https://en.wikipedia.org/wiki/Fizz_buzz)

## In this repository

The implementation of the FizzBuzz test in the [rust language](https://www.rust-lang.org/).

## Benchmark

```
❯ hyperfine -m 1000 target/release/fizzbuss-x86_64-linux
Benchmark #1: target/release/fizzbuss-x86_64-linux
  Time (mean ± σ):       1.8 ms ±   0.4 ms    [User: 1.2 ms, System: 0.7 ms]
  Range (min … max):     1.1 ms …   3.0 ms    1000 runs
```