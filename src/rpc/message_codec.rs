use serde::Serialize;
use tokio_util::bytes::{BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use std::io::Error as IoError;

use super::{encode_message, split_frame};

pub struct MessageCodec;

impl Decoder for MessageCodec {
    type Item = BytesMut;
    type Error = IoError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let (advance, _frame) = match split_frame(src) {
            Ok(Some(t)) => t,
            Ok(None) => return Ok(None), // not enough bytes yet
            Err(e) => return Err(e),
        };

        Ok(Some(src.split_to(advance)))
    }
}

impl<T> Encoder<T> for MessageCodec
where
    T: Serialize,
{
    type Error = IoError;

    fn encode(&mut self, item: T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let bytes = encode_message(&item);
        dst.reserve(bytes.len());
        dst.put_slice(bytes.as_bytes());
        Ok(())
    }
}
