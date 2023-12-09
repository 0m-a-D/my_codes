use std::io;
struct Matrix {
    row: usize,
    col: usize,
    elements: Vec<Vec<f32>>, /* need more understanding */
}
impl Matrix {
    fn input_dim(&mut self) {
        println!("enter number of rows: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        self.row = input.trim().parse().expect("failed to parse input");

        println!("enter number of columns: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        self.col = input.trim().parse().expect("failed to parse input");

        self.elements = vec![vec![0.0; self.col]; self.row];
    }
    fn input_dim_valid(&self, other: &Matrix) {
        if self.col != other.row {
            panic!("Invalid dimensions for matrix multiplication");
        } else {
            println!("Matrix dimensions verified for multiplication. Continue!")
        }
    }
    fn input_elem(&mut self) {
        for i in 0..self.row {
            for j in 0..self.col {
                println!("Enter element at position ({}, {}):", i + 1, j + 1);
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line");
                let element: f32 = input.trim().parse().expect("failed to parse input");
                self.elements[i][j] = element;
            }
        }
    }
    fn matrix_mult(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix {
            row: self.row,
            col: other.col,
            elements: vec![vec![0.0; other.col]; self.row],
        };

        for i in 0..self.row {
            for j in 0..other.col {
                for k in 0..self.col {
                    result.elements[i][j] += self.elements[i][k] * other.elements[k][j];
                }
            }
        }

        result
    }
    fn mult_result(&self) {
        for i in 0..self.row {
            for j in 0..self.col {
                print!("{}\t ", self.elements[i][j]);
            }
            println!();
        }
    }
    fn matrix_trans(&self) {
        for i in 0..self.col {
            for j in 0..self.row {
                print!("{}\t ", self.elements[j][i]);
            }
            println!();
        }
    }
}
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
    result.matrix_trans();
}
