pub fn sum(v: Vec<i32>) -> i32 {
  return v.into_iter().fold(0, |p, c| p + c);
}

pub fn product(v: Vec<i32>) -> i32 {
  return v.into_iter().fold(1, |p: i32, c: i32| -> i32 { p * c });
}
