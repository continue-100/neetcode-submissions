struct NumMatrix {
    matrix: Vec<i32>,
    m: usize,
    n: usize,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        Self {
            matrix: matrix.into_iter().flatten().collect(),
            m,
            n,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut counter = 0;

        for row in (row1 as usize)..=(row2 as usize) {

            let row_offset = row * self.m;
            let start_idx = row_offset + (col1 as usize);
            let end_idx = row_offset + (col2 as usize);

            for val in &self.matrix[start_idx..=end_idx] {
                counter += val;
            }
        }

        counter
    }
}
