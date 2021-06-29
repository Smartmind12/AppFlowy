use bytes::{Buf, Bytes};
use serde::{Deserialize, Serialize};
use std::{fmt, fmt::Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseData {
    Bytes(Vec<u8>),
    None,
}

impl std::fmt::Display for ResponseData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseData::Bytes(bytes) => f.write_fmt(format_args!("{} bytes", bytes.len())),
            ResponseData::None => f.write_str("Empty"),
        }
    }
}

impl std::convert::Into<ResponseData> for String {
    fn into(self) -> ResponseData { ResponseData::Bytes(self.into_bytes()) }
}

impl std::convert::Into<ResponseData> for &'_ String {
    fn into(self) -> ResponseData { ResponseData::Bytes(self.to_owned().into_bytes()) }
}

impl std::convert::Into<ResponseData> for Bytes {
    fn into(self) -> ResponseData { ResponseData::Bytes(self.bytes().to_vec()) }
}

impl std::convert::Into<ResponseData> for &str {
    fn into(self) -> ResponseData { self.to_string().into() }
}
