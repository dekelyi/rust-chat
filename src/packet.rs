use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
// use std::net::SocketAddr;

// type RawPacket = (SocketAddr, Vec<u8>);

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum Packet {
    Msg { msg: String },
    Error { err: String },
}

impl TryFrom<&[u8]> for Packet {
    type Error = anyhow::Error;

    fn try_from(buf: &[u8]) -> anyhow::Result<Packet> {
        let mut de = rmp_serde::Deserializer::new(buf);
        Packet::deserialize(&mut de).context("failed to parse")
    }
}

impl TryInto<Vec<u8>> for Packet {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::<u8>::new();
        let mut ser = rmp_serde::Serializer::new(&mut buf);
        self.serialize(&mut ser)?;
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let buf: Vec<u8> = Packet::Msg {
            msg: "hey".to_string(),
        }
        .try_into()
        .unwrap();
        let excepted = vec![0x92, 0xA3, 0x4D, 0x73, 0x67, 0xA3, 0x68, 0x65, 0x79];
        assert_eq!(buf, excepted);
    }

    #[test]
    fn from() {
        let buf: &[u8] = &[0x92, 0xA3, 0x4D, 0x73, 0x67, 0xA3, 0x68, 0x65, 0x79];
        let msg = Packet::try_from(buf).expect("should parse");
        let excepted = Packet::Msg {
            msg: "hey".to_string(),
        };
        assert_eq!(msg, excepted)
    }
}
