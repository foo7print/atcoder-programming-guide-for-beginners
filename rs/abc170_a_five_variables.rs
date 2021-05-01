use proconio::input;
 
fn main() {
  input! {
    x_1: i32,
    x_2: i32,
    x_3: i32,
    x_4: i32,
    x_5: i32,
  }
  let variables = [x_1, x_2, x_3, x_4, x_5];
  let mut index = 0;
  for (i, n) in variables.iter().enumerate() {
    if *n == 0 {
      index = i + 1;
    }
  }
  println!("{}", index);
}

// fn main() {
//     proconio::input! {
//         a1: i32,
//         a2: i32,
//         a3: i32,
//         a4: i32,
//         a5: i32,
//     }
//     let ans = if a1 == 0 {
//         1
//     } else if a2 == 0 {
//         2
//     } else if a3 == 0 {
//         3
//     } else if a4 == 0 {
//         4
//     } else {
//         5
//     };
//     println!("{}", ans);
// }
