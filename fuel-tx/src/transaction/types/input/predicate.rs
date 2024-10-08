use core::ops::{
    Deref,
    DerefMut,
};
use derivative::Derivative;
use fuel_types::fmt_truncated_hex;

use alloc::vec::Vec;

#[derive(Clone, Default, Derivative)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(fuel_types::canonical::Deserialize, fuel_types::canonical::Serialize)]
#[derivative(Eq, PartialEq, Hash, Debug)]
pub struct PredicateCode {
    #[derivative(Debug(format_with = "fmt_truncated_hex::<16>"))]
    pub bytes: Vec<u8>,
}
impl From<Vec<u8>> for PredicateCode {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}
impl Deref for PredicateCode {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl DerefMut for PredicateCode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

#[cfg(feature = "da-compression")]
impl fuel_compression::Compressible for PredicateCode {
    type Compressed = fuel_compression::RegistryKey;
}
