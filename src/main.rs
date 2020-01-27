use rust_practice::prime::*;

fn main() {
  println!("Hello, World!");
  for n in 2..100 {
    println!("{:02}: {}", n, is_prime(n));
  }
}
