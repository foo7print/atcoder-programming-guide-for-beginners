use proconio::input;

fn main() {
  input! {
    mut a: i32,
    mut b: i32,
    mut c: i32,
  }
  std::mem::swap(&mut a, &mut b);
  std::mem::swap(&mut a, &mut c);
  println!("{} {} {}", a, b, c);
}

