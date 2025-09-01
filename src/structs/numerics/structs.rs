use bincode::{BorrowDecode, Decode, Encode};
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
pub enum Numeric {
    Scaler(f64),
    Vector(Vec<f64>),
    Matrix(Vec<Vec<f64>>),
}

// Newtype wrapper for Arc<RwLock<Numeric>>
#[derive(Debug)]
pub struct SharedNumeric(pub Arc<RwLock<Numeric>>);
impl SharedNumeric {
    pub fn new(numeric: Numeric) -> Self {
        SharedNumeric(Arc::new(RwLock::new(numeric)))
    }
}

impl Serialize for SharedNumeric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Use blocking read for sync trait
        let guard = self.0.blocking_read();
        guard.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SharedNumeric {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let numeric = Numeric::deserialize(deserializer)?;
        Ok(SharedNumeric(Arc::new(RwLock::new(numeric))))
    }
}

impl Clone for SharedNumeric {
    fn clone(&self) -> Self {
        SharedNumeric(Arc::clone(&self.0))
    }
}

impl Encode for SharedNumeric {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        // Use blocking read for sync trait
        tokio::task::block_in_place(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                let guard = self.0.read().await;
                guard.encode(encoder)
            })
        })
    }
}
impl<C> Decode<C> for SharedNumeric {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let numeric = Numeric::decode(decoder)?;
        Ok(SharedNumeric::new(numeric))
    }
}

impl<'de, C> BorrowDecode<'de, C> for SharedNumeric {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let numeric = Numeric::borrow_decode(decoder)?;
        Ok(SharedNumeric::new(numeric))
    }
}
