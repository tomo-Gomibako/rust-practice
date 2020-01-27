pub fn is_prime(n: i32) -> bool {
  for i in 2..=(n as f64).sqrt() as i32 {
    if n % i == 0 {
      return false;
    }
  }
  return true;
}
