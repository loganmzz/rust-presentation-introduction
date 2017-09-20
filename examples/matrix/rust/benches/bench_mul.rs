#![feature(test)]
include!("../src/extra/benchmark.rs");

#[bench]
fn mul_seq_matrix(b: &mut test::Bencher) {
    bench_matrix(b, |a,b| a * b);
}

#[bench]
fn mul_par_matrix(b: &mut test::Bencher) {
    bench_matrix(b, Matrix::mul_par);
}
