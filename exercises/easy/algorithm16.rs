/*
    Rotate Matrix 90 Degrees
    Given a 2D matrix, rotate it 90 degrees in place. 
    You need to perform the rotation without using any additional matrix storage.

    You need to implement the function `rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>)`.
    The function should rotate the input matrix in place.

    Hint: Consider rotating the matrix layer by layer, starting from the outermost layer and working your way inward.
*/

use std::fmt::{self, Display, Formatter};

// (row, col) => (col, max(row) - row - 1)  (2,1) =>(1, 0)
pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let max_row = matrix.len();
    let max_col = matrix[0].len();
    if max_row <= 1 || max_col <= 1 {
        return;
    }

    if max_row > max_col {
        matrix.iter_mut().for_each(|row| row.resize(max_row, 0));
    } else {
        matrix.resize(max_col,vec![0; max_row]);
    }

    for layer in 0..max_row/2 {
        let last = max_row - 1 - layer;
        for i in layer..last {
            let offset = i - layer;
            
            // Save top
            let top = matrix[layer][i];
            
            // Move left to top
            matrix[layer][i] = matrix[last-offset][layer];
            
            // Move bottom to left
            matrix[last-offset][layer] = matrix[last][last-offset];
            
            // Move right to bottom
            matrix[last][last-offset] = matrix[i][last];
            
            // Move top to right
            matrix[i][last] = top;
        }
    }


    matrix.iter_mut().for_each(|row| row.resize(max_row, 0));
    matrix.resize(max_col, vec![0; max_row]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
