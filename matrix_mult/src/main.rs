use matrix_mult::Matrix;
fn main() {
    println!("<dimensions of first matrix>");
    let mut mat1 = Matrix::new();
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
    let mut result = mat1.matrix_mult(&mat2);
    println!("result after matrix multiplication is: ");
    result.print_matrix();
    println!("transpose of resultant matrix is: ");
    result = result.matrix_transpose();
    result.print_matrix();
    // rotating clockwise
    let mut random_mat = Matrix::new();
    println!("enter input of random matrix: ");
    random_mat.input_dim();
    random_mat.input_elem();
    println!("your random matrix is: ");
    random_mat.print_matrix();
    let rotated_random_mat_clockwise = random_mat.rotate_clockwise();
    println!("after rotating random matrix clockwise: ");
    rotated_random_mat_clockwise.print_matrix();
    println!("roting clockwised rotated random matrix back to random matrix:");
    let rotated_random_mat_clockwise_anticlockwise =
        rotated_random_mat_clockwise.rotate_anticlockwise();
    rotated_random_mat_clockwise_anticlockwise.print_matrix();
    //define user defined inputs for number for matrices and perform
    //operations on top of them...
}

//build another function for finding inverse of the matrix...
