fn main() {
  const NUM: u32 = 32;

  println!("{}", f_to_c(NUM));
  println!("{}", c_to_f(NUM));
}

fn f_to_c(x: u32) -> u32 {
  (x-32)*(5/9)
}

fn c_to_f(x: u32) -> u32 {
  x*9/5+32
}