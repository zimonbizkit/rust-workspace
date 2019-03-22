use rand::Rng;

const MATRIX_SIZE: usize = 3;
const UINT_LOWER_RANGE: i32 = 0;
const UINT_UPPER_RANGE: i32 = 101;

#[derive(Debug)]
struct Diagonal {
    values: [i32; MATRIX_SIZE]
}

fn main() {
    // Computes the absolute difference of the sum of the diagonals of two matrices.
    // Would need some refactors as there are a few bad things.
    let mut matrix:[[i32; MATRIX_SIZE]; MATRIX_SIZE] = [[0;MATRIX_SIZE];MATRIX_SIZE];
    let mut secondmatrix:[[i32; MATRIX_SIZE]; MATRIX_SIZE] = [[0;MATRIX_SIZE];MATRIX_SIZE];

    //filling first matrix
    fill_matrix(&mut matrix);
    //filling second matrix
    fill_matrix(&mut secondmatrix);

    println!("First matrix: ");
    print_matrix(matrix);

    println!("Second matrix: ");
    print_matrix(secondmatrix);

    // gathering diagonals
    let mrtldiag: Diagonal = get_diagonal(matrix, true);
    let mltrdiag: Diagonal = get_diagonal(matrix, false);

    let smrtldiag: Diagonal = get_diagonal(secondmatrix, true);
    let smltrdiag: Diagonal = get_diagonal(secondmatrix, false);

    println!("Matrix 1\n\tRTL diagonal: {:?} \n\tLTR diagonal {:?}",mrtldiag,mltrdiag);
    println!("Matrix 2\n\tRTL diagonal: {:?} \n\tLTR diagonal {:?}",smrtldiag,smltrdiag);

    let matrixdiagonalsum = mrtldiag.values.iter().fold(0,|a, &b| a + b) + mltrdiag.values.iter().fold(0,|a, &b| a + b);
    let secondmatrixdiagonalsum = smrtldiag.values.iter().fold(0,|a, &b| a + b) + smltrdiag.values.iter().fold(0,|a, &b| a + b); 
    
    println!("Diagonals sum of first matrix: {} ",matrixdiagonalsum);
    println!("Diagonals sum of second matrix: {} ",secondmatrixdiagonalsum);

    let absresult: i32 = matrixdiagonalsum - secondmatrixdiagonalsum;

    println!("Absolute value of substraction is: {} ",absresult.abs());
}

fn get_diagonal(matrix :[[i32; MATRIX_SIZE]; MATRIX_SIZE], right_to_left :bool) -> Diagonal {
    let mut diagonal_values :[i32; MATRIX_SIZE] = [0;MATRIX_SIZE];

    if right_to_left == true {
        for d in 0..MATRIX_SIZE {
            diagonal_values[d] = matrix[d][d];
        }
    }else{
        let mut i :usize = MATRIX_SIZE -1;
        for d in 0..MATRIX_SIZE{
            diagonal_values[d] = matrix[d][i];
            if i != 0 {
                i = i -1;
            }
        }
    }

    return Diagonal{
        values: diagonal_values
    }
}

fn fill_matrix (matrix :&mut [[i32; MATRIX_SIZE]; MATRIX_SIZE]) {
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            matrix[i][j] = rand::thread_rng().gen_range(UINT_LOWER_RANGE,UINT_UPPER_RANGE);
        }
    }
}

fn print_matrix (matrix :[[i32; MATRIX_SIZE]; MATRIX_SIZE]) {
    for i in 0..MATRIX_SIZE {
        for j in 0..MATRIX_SIZE {
            print!(" {} ",matrix[i][j]);
        }
        println!();
    }
}