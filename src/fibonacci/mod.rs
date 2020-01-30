pub fn fib(n: i32) -> i32 {
  if n < 2 {
    return 1;
  }
  return fib(n - 1) + fib(n - 2);
}

pub fn fib_memo(memo: &mut Vec<i32>, n: i32) -> i32 {
  // println!("{:?}", memo);
  if memo.len() > n as usize {
    return memo[n as usize];
  }
  if n < 2 {
    return 1;
  }
  let result = fib_memo(memo, n - 1) + fib_memo(memo, n - 2);
  memo.push(result);
  return result;
}
