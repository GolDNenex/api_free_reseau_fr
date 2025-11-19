#[allow(unused_imports)]
use crate::{Client, DSLAM, Departement, NRA, TargetType};
use derive_more::Display;

/// Request send by the [`Client`]
#[derive(Debug, Display, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[display("{} {}", target, target_type)]
pub struct Request {
    pub(crate) target: String,
    pub(crate) target_type: TargetType,
}

impl From<NRA> for Request {
    fn from(value: NRA) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::NRA,
        }
    }
}

impl From<&NRA> for Request {
    fn from(value: &NRA) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::NRA,
        }
    }
}

impl From<DSLAM> for Request {
    fn from(value: DSLAM) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::DSLAM,
        }
    }
}

impl From<&DSLAM> for Request {
    fn from(value: &DSLAM) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::DSLAM,
        }
    }
}

impl From<Departement> for Request {
    fn from(value: Departement) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::DEPARTEMENT,
        }
    }
}

impl From<&Departement> for Request {
    fn from(value: &Departement) -> Self {
        Self {
            target: value.0.to_string(),
            target_type: TargetType::DEPARTEMENT,
        }
    }
}
