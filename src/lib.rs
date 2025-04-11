use std::collections::BTreeSet;
use std::io;

pub mod util;

pub fn val2der<V>(value: &V) -> Result<Vec<u8>, io::Error>
where
    V: rasn::Encode,
{
    rasn::der::encode(value).map_err(io::Error::other)
}

#[derive(rasn::AsnType, rasn::Encode)]
pub struct StrToIntMapItem {
    pub skey: String,
    pub ikey: i64,
}

impl StrToIntMapItem {
    pub fn to_der_bytes(&self) -> Result<Vec<u8>, io::Error> {
        val2der(self)
    }
}

pub struct StringSet {
    pub s: BTreeSet<String>,
}

impl StringSet {
    pub fn into_der_bytes(self) -> Result<Vec<u8>, io::Error> {
        let pairs = self.s.into_iter().enumerate().map(|pair| {
            let (i, skey) = pair;
            let ikey: i64 = i as i64;
            StrToIntMapItem { skey, ikey }
        });
        let v: Vec<_> = pairs.collect();
        val2der(&v)
    }
}

pub fn set2der_bytes(s: BTreeSet<String>) -> Result<Vec<u8>, io::Error> {
    StringSet { s }.into_der_bytes()
}
