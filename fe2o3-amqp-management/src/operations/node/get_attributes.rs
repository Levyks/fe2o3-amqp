use std::collections::BTreeMap;

use fe2o3_amqp_types::{
    messaging::{ApplicationProperties, Message},
    primitives::Value,
};

use crate::{
    error::Result,
    operations::{GET_ATTRIBUTES, OPERATION},
    request::MessageSerializer,
};

pub trait GetAttributes {
    fn get_attributes(&self, req: GetAttributesRequest) -> Result<GetAttributesResponse>;
}

/// GET-ATTRIBUTES
///
/// Body:
///
/// No information is carried in the message body therefore any message body is valid and MUST be ignored.
pub struct GetAttributesRequest {
    entity_type: Option<String>,
}

impl MessageSerializer for GetAttributesRequest {
    type Body = ();

    fn into_message(self) -> Message<Self::Body> {
        let mut builder = ApplicationProperties::builder();
        builder = builder.insert(OPERATION, GET_ATTRIBUTES);
        if let Some(entity_type) = self.entity_type {
            builder = builder.insert("entityType", entity_type);
        }
        Message::builder()
            .application_properties(builder.build())
            .value(())
            .build()
    }
}

/// If the request was successful then the statusCode MUST be 200 (OK) and the body of the message
/// MUST contain a map. The keys in the map MUST be the set of Manageable Entity Types for which
/// attribute names are being provided. For any given key, the value MUST be a list of strings
/// representing the attribute names that this Manageable Entity Type possesses. It should be noted
/// that for each entry in the map, the attribute names returned MUST be only those defined by the
/// associated Manageable Entity Type rather than those that are defined by other Manageable Entity
/// Types that extend it. For any given Manageable Entity Type, the set of attribute names returned
/// MUST include every attribute name defined by Manageable Entity Types that it extends, either
/// directly or indirectly.
pub struct GetAttributesResponse {
    map: BTreeMap<Value, Vec<String>>,
}

impl GetAttributesResponse {
    const STATUS_CODE: u16 = 200;
}
