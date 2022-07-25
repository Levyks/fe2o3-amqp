use serde::{de::{self, VariantAccess}, ser};
use serde_amqp::{DeserializeComposite, SerializeComposite};

/// 3.5.10 Delete On Close
/// Lifetime of dynamic node scoped to lifetime of link which caused creation.
/// <type name="delete-on-close" class="composite" source="list" provides="lifetime-policy">
///     <descriptor name="amqp:delete-on-close:list" code="0x00000000:0x0000002b"/>
/// </type>
#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "amqp:delete-on-close:list",
    code = 0x0000_0000_0000_002b,
    encoding = "list"
)]
pub struct DeleteOnClose {}

impl DeleteOnClose {
    /// Creates a new instance of `DeleteOnClose`
    pub fn new() -> Self {
        Self {}
    }
}

impl From<DeleteOnClose> for LifetimePolicy {
    fn from(value: DeleteOnClose) -> Self {
        Self::DeleteOnClose(value)
    }
}

/// 3.5.11 Delete On No Links
/// Lifetime of dynamic node scoped to existence of links to the node
// <type name="delete-on-no-links" class="composite" source="list" provides="lifetime-policy">
//     <descriptor name="amqp:delete-on-no-links:list" code="0x00000000:0x0000002c"/>
// </type>
#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "amqp:delete-on-no-links:list",
    code = 0x0000_0000_0000_002c,
    encoding = "list"
)]
pub struct DeleteOnNoLinks {}

impl DeleteOnNoLinks {
    /// Creates a new instance of `DeleteOnNoLinks`
    pub fn new() -> Self {
        Self {}
    }
}

impl From<DeleteOnNoLinks> for LifetimePolicy {
    fn from(value: DeleteOnNoLinks) -> Self {
        Self::DeleteOnNoLinks(value)
    }
}

/// 3.5.12 Delete On No Messages
/// Lifetime of dynamic node scoped to existence of messages on the node.
/// <type name="delete-on-no-messages" class="composite" source="list" provides="lifetime-policy">
///     <descriptor name="amqp:delete-on-no-messages:list" code="0x00000000:0x0000002d"/>
/// </type>
#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "amqp:delete-on-no-messages:list",
    code = 0x0000_0000_0000_002d,
    encoding = "list"
)]
pub struct DeleteOnNoMessages {}

impl DeleteOnNoMessages {
    /// Creates a new instance of `DeleteOnNoMessages`
    pub fn new() -> Self {
        Self {}
    }
}

impl From<DeleteOnNoMessages> for LifetimePolicy {
    fn from(value: DeleteOnNoMessages) -> Self {
        Self::DeleteOnNoMessages(value)
    }
}

/// 3.5.13 Delete On No Links Or Messages
/// Lifetime of node scoped to existence of messages on or links to the node.
/// <type name="delete-on-no-links-or-messages" class="composite" source="list" provides="lifetime-policy">
///     <descriptor name="amqp:delete-on-no-links-or-messages:list" code="0x00000000:0x0000002e"/>
/// </type>
#[derive(Debug, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "amqp:delete-on-no-links-or-messages:list",
    code = 0x0000_0000_0000_002e,
    encoding = "list"
)]
pub struct DeleteOnNoLinksOrMessages {}

impl DeleteOnNoLinksOrMessages {
    /// Creates a new instance of `DeleteOnNoLinksOrMessages`
    pub fn new() -> Self {
        Self {}
    }
}

impl From<DeleteOnNoLinksOrMessages> for LifetimePolicy {
    fn from(value: DeleteOnNoLinksOrMessages) -> Self {
        Self::DeleteOnNoLinksOrMessages(value)
    }
}

/// The lifetime of a dynamically generated node.
/// Definitionally, the lifetime will never be less than the lifetime
/// of the link which caused its creation, however it is possible to
/// extend the lifetime of dynamically created node using a lifetime
/// policy. The value of this entry MUST be of a type which provides
/// the lifetime-policy archetype. The following standard lifetime-policies
/// are defined below: delete-on-close, delete-on-no-links,
/// delete-on-no-messages or delete-on-no-links-or-messages.
///
/// TODO: impl Into Fields
#[derive(Debug)]
pub enum LifetimePolicy {
    /// 3.5.10 Delete On Close
    /// Lifetime of dynamic node scoped to lifetime of link which caused creation.
    /// <type name="delete-on-close" class="composite" source="list" provides="lifetime-policy">
    ///     <descriptor name="amqp:delete-on-close:list" code="0x00000000:0x0000002b"/>
    /// </type>
    DeleteOnClose(DeleteOnClose),

    /// 3.5.11 Delete On No Links
    /// Lifetime of dynamic node scoped to existence of links to the node
    // <type name="delete-on-no-links" class="composite" source="list" provides="lifetime-policy">
    //     <descriptor name="amqp:delete-on-no-links:list" code="0x00000000:0x0000002c"/>
    // </type>
    DeleteOnNoLinks(DeleteOnNoLinks),

    /// 3.5.12 Delete On No Messages
    /// Lifetime of dynamic node scoped to existence of messages on the node.
    /// <type name="delete-on-no-messages" class="composite" source="list" provides="lifetime-policy">
    ///     <descriptor name="amqp:delete-on-no-messages:list" code="0x00000000:0x0000002d"/>
    /// </type>
    DeleteOnNoMessages(DeleteOnNoMessages),

    /// 3.5.13 Delete On No Links Or Messages
    /// Lifetime of node scoped to existence of messages on or links to the node.
    /// <type name="delete-on-no-links-or-messages" class="composite" source="list" provides="lifetime-policy">
    ///     <descriptor name="amqp:delete-on-no-links-or-messages:list" code="0x00000000:0x0000002e"/>
    /// </type>
    DeleteOnNoLinksOrMessages(DeleteOnNoLinksOrMessages),
}

impl ser::Serialize for LifetimePolicy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LifetimePolicy::DeleteOnClose(value) => value.serialize(serializer),
            LifetimePolicy::DeleteOnNoLinks(value) => value.serialize(serializer),
            LifetimePolicy::DeleteOnNoMessages(value) => value.serialize(serializer),
            LifetimePolicy::DeleteOnNoLinksOrMessages(value) => value.serialize(serializer),
        }
    }
}

enum Field {
    DeleteOnClose,
    DeleteOnNoLinks,
    DeleteOnNoMessages,
    DeleteOnNoLinksOrMessages,
}

struct FieldVisitor {}

impl<'de> de::Visitor<'de> for FieldVisitor {
    type Value = Field;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("enum LifetimePolicy")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = match v {
            "amqp:delete-on-close:list" => Field::DeleteOnClose,
            "amqp:delete-on-no-links:list" => Field::DeleteOnNoLinks,
            "amqp:delete-on-no-messages:list" => Field::DeleteOnNoMessages,
            "amqp:delete-on-no-links-or-messages:list" => Field::DeleteOnNoLinksOrMessages,
            _ => return Err(de::Error::custom("Wrong symbol value for descriptor")),
        };

        Ok(val)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let val = match v {
            0x0000_0000_0000_002b => Field::DeleteOnClose,
            0x0000_0000_0000_002c => Field::DeleteOnNoLinks,
            0x0000_0000_0000_002d => Field::DeleteOnNoMessages,
            0x0000_0000_0000_002e => Field::DeleteOnNoLinksOrMessages,
            _ => {
                return Err(de::Error::custom(format!(
                    "Wrong code value for descriptor, found {:#x?}",
                    v
                )))
            }
        };

        Ok(val)
    }
}

impl<'de> de::Deserialize<'de> for Field {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_identifier(FieldVisitor {})
    }
}

struct Visitor {}

impl<'de> de::Visitor<'de> for Visitor {
    type Value = LifetimePolicy;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("enum LifetimePolicy")
    }

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
        where
            A: de::EnumAccess<'de>, {
        let (field, variant) = data.variant()?;

        match field {
            Field::DeleteOnClose => {
                let value = variant.newtype_variant()?;
                Ok(LifetimePolicy::DeleteOnClose(value))
            },
            Field::DeleteOnNoLinks => {
                let value = variant.newtype_variant()?;
                Ok(LifetimePolicy::DeleteOnNoLinks(value))
            },
            Field::DeleteOnNoMessages => {
                let value = variant.newtype_variant()?;
                Ok(LifetimePolicy::DeleteOnNoMessages(value))
            },
            Field::DeleteOnNoLinksOrMessages => {
                let value = variant.newtype_variant()?;
                Ok(LifetimePolicy::DeleteOnNoLinksOrMessages(value))
            },
        }
    }
}

impl<'de> de::Deserialize<'de> for LifetimePolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        const VARIANTS: &[&str] = &[
            "amqp:delete-on-close:list",
            "amqp:delete-on-no-links:list",
            "amqp:delete-on-no-messages:list",
            "amqp:delete-on-no-links-or-messages:list",
        ];

        deserializer.deserialize_enum("LifetimePolicy", VARIANTS, Visitor {})
    }
}

#[cfg(test)]
mod tests {
    use serde_amqp::{to_vec, from_slice};

    use super::{DeleteOnClose, LifetimePolicy};

    #[test]
    fn test_serialize_enum_and_struct() {
        let s = DeleteOnClose::new();
        let e = LifetimePolicy::DeleteOnClose(DeleteOnClose {});

        let s_buf = to_vec(&s).unwrap();
        let e_buf = to_vec(&e).unwrap();

        assert_eq!(s_buf, e_buf);
    }

    #[test]
    fn test_deserialize_encoded_struct_as_enum() {
        let value = DeleteOnClose::new();
        let buf = to_vec(&value).unwrap();
        let enum_value: LifetimePolicy = from_slice(&buf).unwrap();

        assert!(matches!(enum_value, LifetimePolicy::DeleteOnClose(_)));
    }
}
