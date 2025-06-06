// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.identity.refreshIdentity` namespace.
pub const NSID: &str = "com.atproto.identity.refreshIdentity";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    pub identifier: crate::types::string::AtIdentifier,
}
pub type Input = crate::types::Object<InputData>;
pub type Output = crate::com::atproto::identity::defs::IdentityInfo;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    ///The resolution process confirmed that the handle does not resolve to any DID.
    HandleNotFound(Option<String>),
    ///The DID resolution process confirmed that there is no current DID.
    DidNotFound(Option<String>),
    ///The DID previously existed, but has been deactivated.
    DidDeactivated(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::HandleNotFound(msg) => {
                write!(_f, "HandleNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::DidNotFound(msg) => {
                write!(_f, "DidNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::DidDeactivated(msg) => {
                write!(_f, "DidDeactivated")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
