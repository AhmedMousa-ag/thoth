use crate::{err, structs::numerics::structs::Numeric, warn};

impl Numeric {
    pub fn to_string(&self) -> String {
        match self {
            //TODO, fix it.
            Numeric::Scaler(_) => format!("{:?}", self),
            Numeric::Vector(_) => format!("{:?}", self),
            Numeric::Matrix(_) => format!("{:?}", self),
        }
    }
    pub fn from_string(s: Option<String>) -> Option<Self> {
        if s.is_none() {
            return None;
        }
        let s = s.unwrap();
        if s.is_empty() {
            return None;
        }
        serde_json::from_str::<Self>(&s).ok()
    }
    ///Don't use if your type isn't scaler.
    pub fn get_scaler_value(&self) -> f64 {
        match self {
            Numeric::Scaler(val) => *val,
            _ => {
                let msg = "Expected Scaler variant, will return a zero";
                warn!("{}", msg);
                0.0
            }
        }
    }

    ///Don't use if your type isn't vector.
    pub fn get_vector_value(&self) -> Vec<f64> {
        match self {
            Numeric::Vector(val) => val.clone(),
            _ => {
                let msg = "Expected Vector variant, will return a zero";
                warn!("{}", msg);
                vec![0.0]
            }
        }
    }

    ///Don't use if your type isn't a Matrix.
    pub fn get_matrices_value(&self) -> Vec<Vec<f64>> {
        match self {
            Numeric::Matrix(val) => val.clone(),
            _ => {
                let msg = "Expected Matrix variant, will return a zero";
                warn!("{}", msg);
                vec![vec![0.0]]
            }
        }
    }
}
impl From<f64> for Numeric {
    fn from(val: f64) -> Self {
        Numeric::Scaler(val)
    }
}
impl From<Vec<f64>> for Numeric {
    fn from(vec: Vec<f64>) -> Self {
        Numeric::Vector(vec)
    }
}
impl From<Vec<Vec<f64>>> for Numeric {
    fn from(mat: Vec<Vec<f64>>) -> Self {
        Numeric::Matrix(mat)
    }
}

impl Into<String> for Numeric {
    fn into(self) -> String {
        self.to_string()
    }
}
impl Into<f64> for Numeric {
    fn into(self) -> f64 {
        self.get_scaler_value()
    }
}

impl Into<Vec<f64>> for Numeric {
    fn into(self) -> Vec<f64> {
        self.get_vector_value()
    }
}

impl Into<Vec<Vec<f64>>> for Numeric {
    fn into(self) -> Vec<Vec<f64>> {
        self.get_matrices_value()
    }
}

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
