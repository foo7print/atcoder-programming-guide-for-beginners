use proconio::input;
 
fn main() {
  input! {
    k: i64,
    a: i64,
    b: i64,
  }  
  let mut bool = false;
  for n in a..(b+1) {
    if n % k == 0 {
      bool = true;
    }
  }
  if bool {
    println!("OK");
  } else {
    println!("NG");
  }
}

