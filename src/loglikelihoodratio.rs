/* https://github.com/apache/mahout/blob/08e02602e947ff945b9bd73ab5f0b45863df3e53/math/src/main/java/org/apache/mahout/math/stats/LogLikelihood.java  */
#[inline]
pub fn log_likelihood_ratio(k11: u64, k12: u64, k21: u64, k22: u64) -> f64 {

  let row_entropy = entropy2(k11, k12) + entropy2(k21, k22);
  let column_entropy = entropy2(k11, k21) + entropy2(k12, k22);
  let matrix_entropy = entropy4(k11, k12, k21, k22);
  if row_entropy + column_entropy < matrix_entropy {
    // round off error
    0.0
  } else {
    2.0 * (row_entropy + column_entropy - matrix_entropy)
  }
}

#[inline]
fn x_log_x(x: u64) -> f64 {
  if x == 0 { 0.0 } else { (x as f64) * (x as f64).ln() }
}

#[inline]
fn entropy4(a: u64, b: u64, c: u64, d: u64) -> f64 {
  x_log_x(a + b + c + d) - x_log_x(a) - x_log_x(b) - x_log_x(c) - x_log_x(d)
}

#[inline]
fn entropy2(a: u64, b: u64) -> f64 {
  x_log_x(a + b) - x_log_x(a) - x_log_x(b)
}