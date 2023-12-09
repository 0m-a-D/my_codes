use matrix_mult::Matrix;
fn main() {
    println!("<dimensions of first matrix>");
    let mut mat1 = Matrix {
        row: 0,
        col: 0,
        elements: vec![],
    };
    mat1.input_dim();
    println!("<dimensions of second matrix>");
    let mut mat2 = Matrix {
        row: 0,
        col: 0,
        elements: vec![],
    };
    mat2.input_dim();
    mat1.input_dim_valid(&mat2);
    println!("<Enter elements of first matrix>");
    mat1.input_elem();
    println!("<Enter elements of second matrix>");
    mat2.input_elem();
    let result = mat1.matrix_mult(&mat2);
    println!("result after matrix multiplication is: ");
    result.mult_result();
    println!("transpose of resultant matrix is: ");
    result.matrix_transpose();

    //define user defined inputs for number for matrices and perform
    //operations on top of them...
}

//build another function for finding inverse of the matrix...
