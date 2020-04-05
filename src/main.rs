// use std::time::Instant;
// use rust_practice::prime::*;
// use rust_practice::fibonacci::*;
use rust_practice::closure::*;

fn main() {
  // println!("Hello, World!");

  // for n in 2..20 {
  //   println!("{:02}: {}", n, is_prime(n));
  // }

  // let mut start = Instant::now();
  // for n in 0..=40 {
  //   println!("{:02}: {}", n, fib(n));
  // }
  // println!("{:?}", Instant::now() - start);
  
  // let mut memo = vec![1, 1];
  // start = Instant::now();
  // for n in 0..=40 {
  //   println!("{:02}: {}", n, fib_memo(&mut memo, n));
  // }
  // println!("{:?}", Instant::now() - start);

  println!("{}", sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
  println!("{}", product(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
}
