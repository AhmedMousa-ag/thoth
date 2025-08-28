use crate::{err, structs::numerics::structs::Numeric};

impl std::ops::Add for Numeric {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(a), Numeric::Scaler(b)) => Numeric::Scaler(a + b),
            (Numeric::Vector(a), Numeric::Vector(b)) => {
                if a.len() != b.len() {
                    err!("Vectors must be of the same length for addition.");
                    return Numeric::Vector(vec![]);
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Numeric::Vector(result)
            }
            (Numeric::Matrix(a), Numeric::Matrix(b)) => {
                if a.len() != b.len() || a[0].len() != b[0].len() {
                    err!("Matrices must be of the same dimensions for addition.");
                    return Numeric::Matrix(vec![]);
                }
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(row_a, row_b)| {
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect()
                    })
                    .collect();
                Numeric::Matrix(result)
            }
            _ => {
                err!("Addition not supported between different Numeric types.");
                Numeric::Scaler(0.0) // or handle error as appropriate
            }
        }
    }
}

// Implementation for &Numeric / &Numeric
impl std::ops::Add<&Numeric> for &Numeric {
    type Output = Numeric;
    fn add(self, rhs: &Numeric) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(a), Numeric::Scaler(b)) => Numeric::Scaler(a + b),
            (Numeric::Vector(a), Numeric::Vector(b)) => {
                if a.len() != b.len() {
                    err!("Vectors must be of the same length for addition.");
                    return Numeric::Vector(vec![]);
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Numeric::Vector(result)
            }
            (Numeric::Matrix(a), Numeric::Matrix(b)) => {
                if a.len() != b.len() || a[0].len() != b[0].len() {
                    err!("Matrices must be of the same dimensions for addition.");
                    return Numeric::Matrix(vec![]);
                }
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(row_a, row_b)| {
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect()
                    })
                    .collect();
                Numeric::Matrix(result)
            }
            _ => {
                err!("Addition not supported between different Numeric types.");
                Numeric::Scaler(0.0) // or handle error as appropriate
            }
        }
    }
}

// Implementation for Numeric / &Numeric
impl std::ops::Add<&Numeric> for Numeric {
    type Output = Numeric;
    fn add(self, rhs: &Numeric) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(a), Numeric::Scaler(b)) => Numeric::Scaler(a + b),
            (Numeric::Vector(a), Numeric::Vector(b)) => {
                if a.len() != b.len() {
                    err!("Vectors must be of the same length for addition.");
                    return Numeric::Vector(vec![]);
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Numeric::Vector(result)
            }
            (Numeric::Matrix(a), Numeric::Matrix(b)) => {
                if a.len() != b.len() || a[0].len() != b[0].len() {
                    err!("Matrices must be of the same dimensions for addition.");
                    return Numeric::Matrix(vec![]);
                }
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(row_a, row_b)| {
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect()
                    })
                    .collect();
                Numeric::Matrix(result)
            }
            _ => {
                err!("Addition not supported between different Numeric types.");
                Numeric::Scaler(0.0) // or handle error as appropriate
            }
        }
    }
}

// Implementation for &Numeric / Numeric
impl std::ops::Add<Numeric> for &Numeric {
    type Output = Numeric;
    fn add(self, rhs: Numeric) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(a), Numeric::Scaler(b)) => Numeric::Scaler(a + b),
            (Numeric::Vector(a), Numeric::Vector(b)) => {
                if a.len() != b.len() {
                    err!("Vectors must be of the same length for addition.");
                    return Numeric::Vector(vec![]);
                }
                let result: Vec<f64> = a.iter().zip(b.iter()).map(|(x, y)| x + y).collect();
                Numeric::Vector(result)
            }
            (Numeric::Matrix(a), Numeric::Matrix(b)) => {
                if a.len() != b.len() || a[0].len() != b[0].len() {
                    err!("Matrices must be of the same dimensions for addition.");
                    return Numeric::Matrix(vec![]);
                }
                let result: Vec<Vec<f64>> = a
                    .iter()
                    .zip(b.iter())
                    .map(|(row_a, row_b)| {
                        row_a.iter().zip(row_b.iter()).map(|(x, y)| x + y).collect()
                    })
                    .collect();
                Numeric::Matrix(result)
            }
            _ => {
                err!("Addition not supported between different Numeric types.");
                Numeric::Scaler(0.0) // or handle error as appropriate
            }
        }
    }
}
