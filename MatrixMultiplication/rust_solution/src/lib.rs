type Matrix = Vec<Vec<f64>>;

pub fn multiply(m1: Matrix, m2: Matrix) -> Option<Matrix> {
    if !valid_inputs(&m1, &m2) {
        return None;
    }



    Some(m1)
}

fn dot_product(v1: Vec<f64>, v2: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for (l, r) in v1.iter().zip(v2.iter()) {
        sum += l*r;
    }
    sum
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
