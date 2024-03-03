use std::io;
#[derive(Debug)]
pub struct Matrix {
    pub row: usize,
    pub col: usize,
    pub elements: Vec<Vec<f32>>,
}
impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            row: 0,
            col: 0,
            elements: vec![vec![0.0; 0]; 0],
        }
    }
    pub fn input_dim(&mut self) {
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
    pub fn input_dim_valid(&self, other: &Matrix) -> bool {
        if self.col != other.row {
            panic!("Invalid dimensions for matrix multiplication! Aborting");
        }
        println!("Matrix dimensions verified for multiplication. Continue!");
        true
    }
    pub fn input_elem(&mut self) {
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
    pub fn matrix_mult(&self, other: &Matrix) -> Matrix {
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
    pub fn print_matrix(&self) {
        for i in 0..self.row {
            for j in 0..self.col {
                print!("{}\t ", self.elements[i][j]);
            }
            println!();
        }
    }
    pub fn matrix_transpose(&self) -> Matrix {
        let mut result = Matrix {
            row: self.col,
            col: self.row,
            elements: vec![vec![0.0; self.row]; self.col],
        };

        for i in 0..self.row {
            for j in 0..self.col {
                result.elements[j][i] = self.elements[i][j];
            }
        }

        result
    }
    pub fn rotate_clockwise(&self) -> Matrix {
        let mut rot_clock = self.matrix_transpose();
        for i in 0..rot_clock.row {
            for j in 0..rot_clock.col / 2 {
                let temp = rot_clock.elements[i][j];
                rot_clock.elements[i][j] = rot_clock.elements[i][rot_clock.col - j - 1];
                rot_clock.elements[i][rot_clock.col - j - 1] = temp;
            }
        }
        rot_clock
    }
    pub fn rotate_anticlockwise(&self) -> Matrix {
        let mut rot_anticlock = self.matrix_transpose();
        for j in 0..rot_anticlock.col {
            for i in 0..rot_anticlock.row / 2 {
                let temp = rot_anticlock.elements[i][j];
                rot_anticlock.elements[i][j] = rot_anticlock.elements[rot_anticlock.row - i - 1][j];
                rot_anticlock.elements[rot_anticlock.row - i - 1][j] = temp;
            }
        }
        rot_anticlock
    }
    pub fn matrix_det(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid() {
        let mat1 = Matrix {
            row: 3,
            col: 3,
            elements: vec![vec![0.0; 3]; 3],
        };
        let mat2 = Matrix {
            row: 3,
            col: 3,
            elements: vec![vec![0.0; 3]; 3],
        };
        assert!(mat1.input_dim_valid(&mat2));
    }
}
