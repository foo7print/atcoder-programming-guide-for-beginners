use proconio::input;

fn main() {
  input! {
    a: f64,
    b: f64,
    h: f64,
    m: f64,
  }
  // 分針の，12 時方向に対する角度（ラジアン）
  let minute_hand = m / 60. * 2. * std::f64::consts::PI;
  // 時針の，12 時方向に対する角度（ラジアン）
  let hour_hand = (60. * h + m) / 720. * 2. * std::f64::consts::PI;
  // 分針と時針のなす角度
  let angle = minute_hand - hour_hand;
  // 分針の先と時針の先の距離の 2 乗（余弦定理）
  let doubled = a.powf(2.) + b.powf(2.) - 2. * a * b * angle.cos();
  // 平方根をとって，答え
  let ans = doubled.sqrt();
  println!("{}", ans);
}

