extern crate test;
extern crate examples_matrix;

use examples_matrix::Matrix;

include!("util.rs");

fn bench_matrix<F>(bencher: &mut test::Bencher, size: usize, fun: F) where F: Fn(&Matrix, &Matrix) -> Matrix {
    let base = load_matrix(size);
    fun(&base, &base);
    bencher.iter(move || fun(&base, &base));
}
