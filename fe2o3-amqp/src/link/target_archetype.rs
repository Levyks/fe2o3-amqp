//! Implements verification for Target or Coordinator

use fe2o3_amqp_types::{
    definitions::{self, AmqpError},
    messaging::{Target, TargetArchetype},
    primitives::{Array, Symbol},
};

#[cfg(feature = "transaction")]
use fe2o3_amqp_types::transaction::{Coordinator, TxnCapability};

/// Performs verification on whether the incoming `Target` field complies with the specification
/// or meets the requirement.
pub trait VerifyTargetArchetype {
    fn verify_as_sender(&self, other: &Self) -> Result<(), definitions::Error>;
    fn verify_as_receiver(&self, other: &Self) -> Result<(), definitions::Error>;
}

impl VerifyTargetArchetype for Target {
    fn verify_as_sender(&self, other: &Self) -> Result<(), definitions::Error> {
        // The address of the source MUST be set when sent on a attach frame sent by the receiving
        // link endpoint where the dynamic flag is set to true (that is where the receiver has
        // created an addressable node at the request of the sender and is now communicating the
        // address of that created node).
        if other.dynamic && other.address.is_none() {
            return Err(definitions::Error::new(
                AmqpError::NotAllowed,
                "The address of the source MUST be set when sent on a attach frame sent by the receiving link endpoint where the dynamic flag is set to true".to_string(),
                None
            ));
        } else if !other.dynamic && other.dynamic_node_properties.is_some() {
            // If the dynamic field is not set to true this field MUST be left unset.
            return Err(definitions::Error::new(
                AmqpError::NotAllowed,
                "If the dynamic field is not set to true this field MUST be left unset."
                    .to_string(),
                None,
            ));
        }

        Ok(())
    }

    fn verify_as_receiver(&self, other: &Self) -> Result<(), definitions::Error> {
        // The address of the target MUST NOT be set when sent on a attach frame sent by the sending
        // link endpoint where the dynamic flag is set to true (that is where the sender is
        // requesting the receiver to create an addressable node).
        if other.dynamic && other.address.is_some() {
            return Err(definitions::Error::new(
                AmqpError::NotAllowed,
                "The address of the target MUST NOT be set when sent on a attach frame sent by the sending link endpoint where the dynamic flag is set to true".to_string(),
                None
            ));
        } else if !other.dynamic && other.dynamic_node_properties.is_some() {
            // If the dynamic field is not set to true this field MUST be left unset.
            return Err(definitions::Error::new(
                AmqpError::NotAllowed,
                "If the dynamic field is not set to true this field MUST be left unset."
                    .to_string(),
                None,
            ));
        }

        Ok(())
    }
}

#[cfg(feature = "transaction")]
impl VerifyTargetArchetype for Coordinator {
    fn verify_as_sender(&self, other: &Self) -> Result<(), definitions::Error> {
        // Note that it is the responsibility of the transaction controller to verify that the
        // capabilities of the controller meet its requirements.
        match (&self.capabilities, &other.capabilities) {
            (Some(desired), Some(provided)) => {
                for cap in desired.0.iter() {
                    if !provided.0.contains(cap) {
                        return Err(definitions::Error::new(
                            AmqpError::InternalError,
                            "Desired transaction capabilities are not supported".to_string(),
                            None,
                        ));
                    }
                }
                Ok(())
            }
            (Some(desired), None) => {
                if desired.0.is_empty() {
                    Ok(())
                } else {
                    Err(definitions::Error::new(
                        AmqpError::InternalError,
                        "Desired transaction capabilities are not supported".to_string(),
                        None,
                    ))
                }
            }
            (None, Some(_)) | (None, None) => Ok(()),
        }
    }

    fn verify_as_receiver(&self, _other: &Self) -> Result<(), definitions::Error> {
        // Note that it is the responsibility of the transaction controller to verify that the
        // capabilities of the controller meet its requirements.
        Ok(())
    }
}

pub trait TargetArchetypeCapabilities {
    type Capability;

    fn capabilities(&self) -> &Option<Array<Self::Capability>>;

    fn capabilities_mut(&mut self) -> &mut Option<Array<Self::Capability>>;
}

impl TargetArchetypeCapabilities for Target {
    type Capability = Symbol;

    fn capabilities(&self) -> &Option<Array<Symbol>> {
        &self.capabilities
    }

    fn capabilities_mut(&mut self) -> &mut Option<Array<Symbol>> {
        &mut self.capabilities
    }
}

#[cfg(feature = "transaction")]
impl TargetArchetypeCapabilities for Coordinator {
    type Capability = TxnCapability;

    fn capabilities(&self) -> &Option<Array<TxnCapability>> {
        &self.capabilities
    }

    fn capabilities_mut(&mut self) -> &mut Option<Array<TxnCapability>> {
        &mut self.capabilities
    }
}

pub trait VariantOfTargetArchetype {
    fn is_target(&self) -> bool;
    fn is_coordinator(&self) -> bool;
}

impl VariantOfTargetArchetype for Target {
    fn is_target(&self) -> bool {
        true
    }

    fn is_coordinator(&self) -> bool {
        false
    }
}

#[cfg(feature = "transaction")]
impl VariantOfTargetArchetype for Coordinator {
    fn is_target(&self) -> bool {
        false
    }

    fn is_coordinator(&self) -> bool {
        true
    }
}

impl VariantOfTargetArchetype for TargetArchetype {
    fn is_target(&self) -> bool {
        match self {
            TargetArchetype::Target(_) => true,
            #[cfg(feature = "transaction")]
            TargetArchetype::Coordinator(_) => false,
        }
    }

    fn is_coordinator(&self) -> bool {
        match self {
            TargetArchetype::Target(_) => true,
            #[cfg(feature = "transaction")]
            TargetArchetype::Coordinator(_) => false,
        }
    }
}

/// Extension trait for TargetArchetypes
pub trait TargetArchetypeExt: VerifyTargetArchetype + TargetArchetypeCapabilities + VariantOfTargetArchetype {

}

impl<T> TargetArchetypeExt for T where T: VerifyTargetArchetype + TargetArchetypeCapabilities + VariantOfTargetArchetype {}
