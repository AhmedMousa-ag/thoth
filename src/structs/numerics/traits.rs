use crate::{structs::numerics::structs::Numeric, warn};

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
