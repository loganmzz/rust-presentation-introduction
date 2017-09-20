extern crate examples_matrix;

use examples_matrix::Matrix;

fn main() {
    let a = Matrix::from_vec(
        vec!(
            vec!(1, 2, 4)
        )
    );

    let b = Matrix::from_vec(
        vec!(
            vec!(8, 16, 32)
        )
    );

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a + b = {:?}", &a + &b);
}
