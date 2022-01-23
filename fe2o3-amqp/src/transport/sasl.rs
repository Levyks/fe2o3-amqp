use serde::{ser, de::{self, VariantAccess}, Serialize, Deserialize};

use fe2o3_amqp_types::sasl::{SaslChallenge, SaslInit, SaslMechanisms, SaslOutcome, SaslResponse};
use serde_amqp::read::IoReader;
use tokio_util::codec::{Encoder, Decoder};

use super::Error;

// pub struct Frame {
//     pub body: FrameBody,
// }

/// TODO: impl Serialize and Deserialize
#[derive(Debug)]
pub enum Frame {
    Mechanisms(SaslMechanisms),
    Init(SaslInit),
    Challenge(SaslChallenge),
    Response(SaslResponse),
    Outcome(SaslOutcome),
}

pub struct FrameCodec {}

impl Encoder<Frame> for FrameCodec {
    type Error = Error;

    fn encode(&mut self, item: Frame, dst: &mut bytes::BytesMut) -> Result<(), Self::Error> {
        use bytes::BufMut;
        use serde_amqp::ser::Serializer;

        let mut serializer = Serializer::from(dst.writer());
        item.serialize(&mut serializer)?;
        Ok(())
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        use bytes::Buf;
        use serde_amqp::de::Deserializer;

        let reader = IoReader::new(src.reader());
        let mut deserializer = Deserializer::new(reader);
        let frame: Frame = Deserialize::deserialize(&mut deserializer)?;
        Ok(Some(frame))
    }
}

impl ser::Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        match self {
            Frame::Mechanisms(value) => value.serialize(serializer),
            Frame::Init(value) => value.serialize(serializer),
            Frame::Challenge(value) => value.serialize(serializer),
            Frame::Response(value) => value.serialize(serializer),
            Frame::Outcome(value) => value.serialize(serializer)
        }
    }
}

enum Field {
    Mechanisms,
    Init,
    Challenge,
    Response,
    Outcome,
}

struct FieldVisitor {}

impl<'de> de::Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("SASL FrameBody variant identifier")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error, 
    {
        let val = match v {
            "amqp:sasl-mechanisms:list" => Field::Mechanisms,
            "amqp:sasl-init:list" => Field::Init,
            "amqp:sasl-challenge:list" => Field::Challenge,
            "amqp:sasl-response:list" => Field::Response,
            "amqp:sasl-outcome:list" => Field::Outcome,
            _ => return Err(de::Error::custom("Wrong symbol value for SASL frame body descriptor"))
        };
        Ok(val)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error, 
    {
        let val = match v {
            0x0000_0000_0000_0040 => Field::Mechanisms,
            0x0000_0000_0000_0041 => Field::Init,
            0x0000_0000_0000_0042 => Field::Challenge,
            0x0000_0000_0000_0043 => Field::Response,
            0x0000_0000_0000_0044 => Field::Outcome,
            _ => return Err(de::Error::custom("Wrong code value for SASL frame body descriptor"))
        };
        Ok(val)
    }
}

impl<'de> de::Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        deserializer.deserialize_identifier(FieldVisitor {})
    }
}

struct Visitor {}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = Frame;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("enum SASL FrameBody")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: de::EnumAccess<'de>, 
    {
        let (val, variant) = data.variant()?;

        match val {
            Field::Mechanisms => {
                let value = variant.newtype_variant()?;
                Ok(Frame::Mechanisms(value))
            },
            Field::Init => {
                let value = variant.newtype_variant()?;
                Ok(Frame::Init(value))
            },
            Field::Challenge => {
                let value = variant.newtype_variant()?;
                Ok(Frame::Challenge(value))
            },
            Field::Response => {
                let value = variant.newtype_variant()?;
                Ok(Frame::Response(value))
            },
            Field::Outcome => {
                let value = variant.newtype_variant()?;
                Ok(Frame::Outcome(value))
            }
        }
    }
}

impl<'de> de::Deserialize<'de> for Frame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> 
    {
        const VARIANTS: &'static [&'static str] = &[
            "amqp:sasl-mechanisms:list",
            "amqp:sasl-init:list",
            "amqp:sasl-challenge:list",
            "amqp:sasl-response:list",
            "amqp:sasl-outcome:list",
        ];
        deserializer.deserialize_enum("sasl::FrameBody", VARIANTS, Visitor {})
    }
}

#[cfg(test)]
mod tests {
    use fe2o3_amqp_types::{sasl::SaslMechanisms, primitives::Symbol};
    use serde_amqp::{to_vec, from_slice};

    #[test]
    fn test_serialize_frame_body() {
        let mechanism = SaslMechanisms {
            sasl_server_mechanisms: vec![Symbol::from("PLAIN")]
        };
        let buf = to_vec(&mechanism).unwrap();
        println!("{:#x?}", buf);
        let deserialized: super::Frame = from_slice(&buf).unwrap();
        println!("{:?}", deserialized);
    }
}