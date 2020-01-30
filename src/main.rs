use std::time::Instant;
// use rust_practice::prime::*;
use rust_practice::fibonacci::*;

fn main() {
  // println!("Hello, World!");

  // for n in 2..20 {
  //   println!("{:02}: {}", n, is_prime(n));
  // }

  let mut start = Instant::now();
  for n in 0..=40 {
    println!("{:02}: {}", n, fib(n));
  }
  println!("{:?}", Instant::now() - start);
  
  let mut memo = vec![1, 1];
  start = Instant::now();
  for n in 0..=40 {
    println!("{:02}: {}", n, fib_memo(&mut memo, n));
  }
  println!("{:?}", Instant::now() - start);
}
