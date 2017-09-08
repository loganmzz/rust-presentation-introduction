#![feature(test)]
include!("../src/extra/benchmark.rs");

#[bench]
fn add_seq_matrix_4096(b: &mut test::Bencher) {
    bench_matrix(b, 2*4096, |a,b| a + b);
}

#[bench]
fn add_par_matrix_4096(b: &mut test::Bencher) {
    bench_matrix(b, 2*4096, Matrix::add_par);
}
