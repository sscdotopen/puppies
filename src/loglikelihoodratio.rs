/* https://github.com/apache/mahout/blob/08e02602e947ff945b9bd73ab5f0b45863df3e53/math/src/main/java/org/apache/mahout/math/stats/LogLikelihood.java  */

#[inline(always)]
pub fn log_likelihood_ratio(k11: u64, k12: u64, k21: u64, k22: u64) -> f64 {

  /* Thank you Frank - https://www.reddit.com/r/rust/comments/6qmnbo/why_is_my_scala_program_twice_as_fast_as_my_rust/dl0x1bj/ */

  let xlx11 = x_log_x(k11);
  let xlx12 = x_log_x(k12);
  let xlx21 = x_log_x(k21);
  let xlx22 = x_log_x(k22);

  let xlx_sum = xlx11 + xlx12 + xlx21 + xlx22;

  let xlx_row = x_log_x(k11 + k12) + x_log_x(k21 + k22) - xlx_sum;
  let xlx_col = x_log_x(k11 + k21) + x_log_x(k12 + k22) - xlx_sum;

  let xlx_mat = x_log_x(k11 + k12 + k21 + k22) - xlx_sum;

  if xlx_row + xlx_col < xlx_mat {
    // round off error
    0.0
  } else {
    2.0 * (xlx_row + xlx_col - xlx_mat)
  }
}

#[inline(always)]
fn x_log_x(x: u64) -> f64 {
  if x == 0 { 0.0 } else { (x as f64) * (x as f64).ln() }
}
