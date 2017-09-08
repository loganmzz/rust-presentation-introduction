#![feature(test)]
include!("../src/extra/benchmark.rs");

#[bench]
fn mul_seq_matrix_256(b: &mut test::Bencher) {
    bench_matrix(b, 256, |a,b| a * b);
}

#[bench]
fn mul_par_matrix_256(b: &mut test::Bencher) {
    bench_matrix(b, 256, Matrix::mul_par);
}
