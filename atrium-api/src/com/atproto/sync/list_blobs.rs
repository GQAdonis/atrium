// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.sync.listBlobs` namespace.
pub const NSID: &str = "com.atproto.sync.listBlobs";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    ///The DID of the repo.
    pub did: crate::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<crate::types::LimitedNonZeroU16<1000u16>>,
    ///Optional revision of the repo to list blobs since.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub since: core::option::Option<crate::types::string::Tid>,
}
pub type Parameters = crate::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub cids: Vec<crate::types::string::Cid>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
}
pub type Output = crate::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    RepoNotFound(Option<String>),
    RepoTakendown(Option<String>),
    RepoSuspended(Option<String>),
    RepoDeactivated(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::RepoNotFound(msg) => {
                write!(_f, "RepoNotFound")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoTakendown(msg) => {
                write!(_f, "RepoTakendown")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoSuspended(msg) => {
                write!(_f, "RepoSuspended")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::RepoDeactivated(msg) => {
                write!(_f, "RepoDeactivated")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
