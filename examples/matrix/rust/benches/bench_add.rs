#![feature(test)]
include!("../src/extra/benchmark.rs");

#[bench]
fn add_seq_matrix(b: &mut test::Bencher) {
    bench_matrix(b, |a,b| a + b);
}

#[bench]
fn add_par_matrix(b: &mut test::Bencher) {
    bench_matrix(b, Matrix::add_par);
}
