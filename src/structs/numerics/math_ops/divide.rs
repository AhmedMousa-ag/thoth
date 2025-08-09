use crate::{err, structs::numerics::structs::Numeric};


impl std::ops::Div for Numeric {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(l), Numeric::Scaler(r)) => Numeric::Scaler(l / r),
            (Numeric::Vector(l), Numeric::Vector(r)) => {
                if l.len() != r.len() {
                    let msg = "Vector lengths must match";
                    err!("{}", msg; panic = true);
                    unreachable!("{}", msg);
                }
                Numeric::Vector(l.into_iter().zip(r).map(|(l, r)| l / r).collect())
            }
            (Numeric::Matrix(l), Numeric::Matrix(r)) => {
                if l.len() != r.len() || l[0].len() != r[0].len() {
                    let msg = "Matrix dimensions must match";
                    err!("{}", msg; panic = true);
                    unreachable!("{}", msg);
                }
                Numeric::Matrix(
                    //TODO enhance this, it can be done in parallel.
                    l.into_iter()
                        .zip(r)
                        .map(|(l_row, r_row)| {
                            l_row.into_iter().zip(r_row).map(|(l, r)| l / r).collect()
                        })
                        .collect(),
                )
            }
            _ => {
                let msg = "Incompatible Numeric types";
                err!("{}", msg; panic = true);
                unreachable!("{}", msg);
            }
        }
    }
}

// Implementation for &Numeric / &Numeric
impl std::ops::Div<&Numeric> for &Numeric {
    type Output = Numeric;
    fn div(self, rhs: &Numeric) -> Self::Output {
        match (self, rhs) {
            (Numeric::Scaler(l), Numeric::Scaler(r)) => Numeric::Scaler(l / r),
            (Numeric::Vector(l), Numeric::Vector(r)) => {
                if l.len() != r.len() {
                    let msg = "Vector lengths must match";
                    err!("{}", msg; panic = true);
                    unreachable!("{}", msg);
                }
                Numeric::Vector(l.iter().zip(r.iter()).map(|(l, r)| l / r).collect())
            }
            (Numeric::Matrix(l), Numeric::Matrix(r)) => {
                if l.len() != r.len() || l[0].len() != r[0].len() {
                    let msg = "Matrix dimensions must match";
                    err!("{}", msg; panic = true);
                    unreachable!("{}", msg);
                }
                Numeric::Matrix(
                    //TODO enhance this, it can be done in parallel.
                    l.iter()
                        .zip(r.iter())
                        .map(|(l_row, r_row)| {
                            l_row.iter().zip(r_row.iter()).map(|(l, r)| l / r).collect()
                        })
                        .collect(),
                )
            }
            _ => {
                let msg = "Incompatible Numeric types";
                err!("{}", msg; panic = true);
                unreachable!("{}", msg);
            }
        }
    }
}

// Implementation for Numeric / &Numeric
impl std::ops::Div<&Numeric> for Numeric {
    type Output = Numeric;
    fn div(self, rhs: &Numeric) -> Self::Output {
        &self / rhs
    }
}

// Implementation for &Numeric / Numeric
impl std::ops::Div<Numeric> for &Numeric {
    type Output = Numeric;
    fn div(self, rhs: Numeric) -> Self::Output {
        self / &rhs
    }
}
