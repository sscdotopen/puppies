#[macro_use]
extern crate bencher;
extern crate rand;
extern crate puppies;

use bencher::Bencher;
use rand::{Rng,SeedableRng,XorShiftRng};

use puppies::llr::x_log_x;
use puppies::llr::logarithms_table;
use puppies::llr::log_likelihood_ratio;

#[inline(always)]
fn llr_cse(k11: u64, k12: u64, k21: u64, k22: u64) -> f64 {

    let xlx_all = x_log_x(k11 + k12 + k21 + k22);

    let row_entropy = xlx_all - x_log_x(k11 + k12) - x_log_x(k21 + k22);
    let column_entropy = xlx_all - x_log_x(k11 + k21) - x_log_x(k12 + k22);

    let matrix_entropy = xlx_all - x_log_x(k11) - x_log_x(k12) -  x_log_x(k21) - x_log_x(k22);

    if row_entropy + column_entropy < matrix_entropy {
        0.0
    } else {
        2.0 * (row_entropy + column_entropy - matrix_entropy)
    }
}

#[inline(always)]
fn llr_no_cse(k11: u64, k12: u64, k21: u64, k22: u64) -> f64 {
    let row_entropy = entropy2(k11 + k12, k21 + k22);
    let column_entropy = entropy2(k11 + k21, k12 + k22);
    let matrix_entropy = entropy4(k11, k12, k21, k22);
    if row_entropy + column_entropy < matrix_entropy {
        0.0
    } else {
        2.0 * (row_entropy + column_entropy - matrix_entropy)
    }
}

#[inline(always)]
fn entropy2(a: u64, b: u64) -> f64 {
    x_log_x(a + b) - x_log_x(a) - x_log_x(b)
}

#[inline(always)]
fn entropy4(a: u64, b: u64, c: u64, d: u64) -> f64 {
    x_log_x(a + b + c + d) - x_log_x(a) - x_log_x(b) - x_log_x(c) - x_log_x(d)
}


const NUM_REPETITIONS: u32 = 1000;
const FIXED_SEED: [u32; 4] = [1, 2, 3, 4];
const MAX_OBSERVATIONS: u64 = 500;
const MAX_COOCCURRENCES: u64 = MAX_OBSERVATIONS * MAX_OBSERVATIONS;


fn bench_llr_cse_pre(bench: &mut Bencher) {

    let mut rng = XorShiftRng::from_seed(FIXED_SEED);

    let logs = logarithms_table((MAX_COOCCURRENCES + MAX_OBSERVATIONS) as usize);

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            bencher::black_box(log_likelihood_ratio(
                rng.gen_range(0, MAX_COOCCURRENCES),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.next_u64(),
                &logs));
        }
    })
}

fn bench_llr_cse(bench: &mut Bencher) {

    let mut rng = XorShiftRng::from_seed(FIXED_SEED);

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            bencher::black_box(llr_cse(
                rng.gen_range(0, MAX_COOCCURRENCES),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.next_u64()));
        }
    })
}

fn bench_llr_no_cse(bench: &mut Bencher) {

    let mut rng = XorShiftRng::from_seed(FIXED_SEED);

    bench.iter(|| {
        for _ in 0..NUM_REPETITIONS {
            bencher::black_box(llr_no_cse(
                rng.gen_range(0, MAX_COOCCURRENCES),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.gen_range(0, MAX_OBSERVATIONS),
                rng.next_u64()));
        }
    })
}

benchmark_group!(benches, bench_llr_no_cse, bench_llr_cse, bench_llr_cse_pre);
benchmark_main!(benches);