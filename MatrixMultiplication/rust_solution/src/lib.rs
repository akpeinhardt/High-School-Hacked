type Matrix = Vec<Vec<f64>>;

pub fn multiply(m1: Matrix, m2: Matrix) -> Option<Matrix> {
    if !valid_inputs(&m1, &m2) {
        return None;
    }

    let mut result: Matrix = Vec::with_capacity(m1.len());
    for _ in 0..m1.len() {
        result.push(Vec::with_capacity(m2[0].len()));
    }
    for (row, col) in (0..m1[0].len()).zip(0..m2.len()) {
        result[row][col] = dot_product(&m1[row], &get_nth_col(&m2, col));
    }

    Some(m1)
}

fn dot_product(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for (l, r) in v1.iter().zip(v2.iter()) {
        sum += l*r;
    }
    sum
}

fn get_nth_col(m: &Matrix, n: usize) -> Vec<f64> {
    let mut result = Vec::new();

    for row in m.iter() {
        result.push(row[n]);
    }

    result
}

fn valid_inputs(m1: &Matrix, m2: &Matrix) -> bool {
    if m1[0].len() != m2.len() {
        return false;
    }

    let row_len = m1[0].len();
    for row in m1.iter() {
        if row.len() != row_len {
            return false;
        }
    }

    let row_len = m2[0].len();
    for row in m2.iter() {
        if row.len() != row_len {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        assert_eq!(multiply(vec![vec![1.0,2.0], vec![3.0,4.0]], vec![vec![1.0,2.0], vec![3.0,4.0]]).unwrap(), vec![vec![7.0, 10.0], vec![15.0, 22.0]]);
    }
}
