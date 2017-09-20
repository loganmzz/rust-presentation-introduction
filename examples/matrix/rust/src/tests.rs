use Matrix;

include!("extra/util.rs");

#[test]
fn load_4096() {
    let size = 4096;
    let m = load_matrix(size);
    assert_eq!(size, m.cols());
    assert_eq!(size, m.rows());
    for col in 0..m.cols() {
        for row in 0..m.rows() {
            assert_eq!(col as i32, m[(col, row)], "at ({}, {})", col, row);
        }
    }
}

fn assert_sum_matrix<F>(size: usize, op: F) where F: FnOnce(&Matrix, &Matrix) -> Matrix {
    let base = load_matrix(size);
    let m = op(&base, &base);
    for col in 0..m.cols() {
        for row in 0..m.rows() {
            assert_eq!((col * 2) as i32, m[(col, row)], "at ({}, {})", col, row);
        }
    }
}

#[test]
fn add_par_4096() {
    assert_sum_matrix(4096, Matrix::add_par);
}

#[test]
fn add_seq_4096() {
    assert_sum_matrix(4096, |rhs,lhs| rhs + lhs);
}

fn assert_matrix_eq_vec(actual: Matrix, expected: Vec<Vec<i32>>) {
    assert_eq!(Matrix::from_vec(expected), actual);
}

fn assert_matrix_eq(actual: Matrix, expected: Matrix) {
    assert_eq!(expected, actual);
}

fn m(data: i32) -> Matrix {
    let mut m = Matrix::new(1, 1);
    m[(0, 0)] = data;
    m
}

fn square(d00: i32, d10: i32, d01: i32, d11: i32) -> Matrix {
    Matrix::from_vec(vec!(
        vec!(d00, d10),
        vec!(d01, d11)
    ))
}

#[test]
fn sum_0_0() {
    assert_matrix_eq_vec(&m(0) + &m(0), vec![
        vec!(0)
    ]);
}

#[test]
fn sum_0_1() {
    assert_matrix_eq_vec(&m(0) + &m(1), vec![
        vec!(1)
    ]);
}

#[test]
fn sum_1_0() {
    assert_matrix_eq_vec(&m(1) + &m(0), vec![
        vec!(1)
    ]);
}

#[test]
fn sum_two_squares() {
    assert_matrix_eq(&square(1, 2, 4, 8) + &square(16, 32, 64, 128), square(17, 34, 68, 136));
}

#[test]
fn mul_two_squares() {
    assert_eq!(square(19, 22, 43, 50), &square(1, 2, 3, 4) * &square(5, 6, 7, 8));
}

fn assert_mul_identity<F>(size: usize, fun: F) where F: FnOnce(&Matrix, &Matrix) -> Matrix {
    let base = load_matrix(size);
    let id   = Matrix::identity(size);
    assert_eq!(base, fun(&base, &id));
}

#[test]
fn mul_matrix_64() {
    assert_mul_identity(64, |a,b| a * b);
}

#[test]
fn mul_par_matrix_64() {
    assert_mul_identity(64, Matrix::mul_par);
}

#[test]
fn mul_par_equal_mul_64() {
    let base = load_matrix(64);
    assert_eq!(&base * &base, base.mul_par(&base));
}