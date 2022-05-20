fn main() {
  println!("{}", fib(0, 1, 0, 1))
}

fn fib(x:i32, y:i32, ci:i32, i:i32) -> i32 {
  if ci < i {
    fib(y, x+y, ci+1, i)
  } else {
    x
  }
}