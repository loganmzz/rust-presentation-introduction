extern crate test;
extern crate examples_matrix;
extern crate rayon;

use examples_matrix::Matrix;

include!("util.rs");

fn bench_matrix<F>(bencher: &mut test::Bencher, fun: F) where F: Fn(&Matrix, &Matrix) -> Matrix {
    let base = load_matrix(512);
    fun(&base, &base);
    bencher.iter(move || fun(&base, &base));
}
