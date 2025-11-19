#![doc = include_str!("../README.md")]

use derive_more::{Deref, DerefMut, Display, Error, From};

pub(crate) const BASE_URL: &str = "https://www.free-reseau.fr/outils/api";

pub(crate) mod client;
pub(crate) mod parser;
pub(crate) mod request;

pub use client::Client;
pub use request::*;

/// Returned by the [`Client`] is a error occured
#[derive(Debug, Display, From, Error)]
pub enum Error {
    ///Return if a [`reqwest`] occured
    #[display("{_0}")]
    Reqwest(#[from] reqwest::Error),
    ///Return when target is non existent
    #[display("Nonexistent {_0} was provided `{_1}`")]
    NonExistent(TargetType, String),
}

/// Store a NRA as a String. For example: `mon75`
#[derive(Debug, Clone, Display, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NRA(pub String);

impl NRA {
    /// Create a new NRA with the provided fields
    pub fn new(target: impl Into<String>, departement_number: u16) -> Self {
        Self(format!("{}{}", target.into(), departement_number,))
    }
}
impl From<&str> for NRA {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// Store a DSLAM String. For example: `mon75-1`
#[derive(Debug, Clone, Display, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DSLAM(pub String);

impl DSLAM {
    /// Create a new DSLAM with the provided fields
    pub fn new(target: impl Into<String>, departement_number: u16, target_id: u16) -> Self {
        Self(format!(
            "{}{}-{}",
            target.into(),
            departement_number,
            target_id
        ))
    }
}

impl From<&str> for DSLAM {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// Store a Departement. For example: `75`
#[derive(Debug, Clone, Copy, Display, Deref, DerefMut, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Departement(pub u16);

impl Departement {
    /// Create a new Departement with the provided field
    pub fn new(target: u16) -> Self {
        Self(target)
    }
}

/// Represent the 3 types of request possible
#[derive(Debug, Display, From, Error, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TargetType {
    #[display("DSLAM")]
    DSLAM,
    #[display("NRA")]
    NRA,
    #[display("DEPARTEMENT")]
    DEPARTEMENT,
}

/// Status of a [`Departement`]
#[derive(Debug, Display, From, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DepartementStatus {
    #[display("OK")]
    OK,
    #[display("DEGRADED")]
    DEGRADED,
}

impl From<bool> for DepartementStatus {
    fn from(value: bool) -> Self {
        match value {
            true => DepartementStatus::OK,
            false => DepartementStatus::DEGRADED,
        }
    }
}

/// Status of a [`NRA`]
#[derive(Debug, Display, From, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NRAStatus {
    #[display("OK")]
    OK,
    #[display("DEGRADED")]
    DEGRADED,
}

impl From<bool> for NRAStatus {
    fn from(value: bool) -> Self {
        match value {
            true => NRAStatus::OK,
            false => NRAStatus::DEGRADED,
        }
    }
}

/// Status of a [`DSLAM`]
#[derive(Debug, Display, From, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DSLAMStatus {
    #[display("ONLINE")]
    ONLINE,
    #[display("OFFLINE")]
    OFFLINE,
}

impl From<bool> for DSLAMStatus {
    fn from(value: bool) -> Self {
        match value {
            true => DSLAMStatus::ONLINE,
            false => DSLAMStatus::OFFLINE,
        }
    }
}

/// Result of a API request
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Response {
    /// Reponse for a [`DSLAM`]
    DSLAM {
        /// target fullname
        target: String,
        target_status: DSLAMStatus,
    },
    /// Reponse for a [`NRA`]
    NRA {
        /// target fullname
        target: String,
        target_status: NRAStatus,
    },
    /// Reponse for a [`Departement`]
    DEPARTEMENT {
        /// target fullname
        target: String,
        target_status: DepartementStatus,
    },
    /// Error returned
    Err {
        /// target fullname
        target: String,
        /// target type
        target_type: TargetType,
        /// erro type
        error_message: String,
    },
}
