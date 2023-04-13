use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;

pub(crate) mod github;

/// Type alias to represent a service name.
pub(crate) type ServiceName = &'static str;

/// Trait that defines some operations a service handler must support.
#[async_trait]
pub(crate) trait ServiceHandler {
    /// Return a summary of the changes detected in the service's state as
    /// defined in the configuration from the base to the head reference.
    async fn get_changes_summary(&self, head_ref: &str) -> Result<ChangesSummary>;

    /// Apply the changes needed so that the actual state (as defined in the
    /// service) matches the desired state (as defined in the configuration).
    async fn reconcile(&self) -> Result<ChangesApplied>;
}

/// Type alias to represent a service handler trait object.
pub(crate) type DynServiceHandler = Box<dyn ServiceHandler + Send + Sync>;

/// Represents a summary of changes between two versions of the service's state.
pub(crate) struct ChangesSummary {
    pub changes: Vec<DynChange>,
    pub base_ref_config_status: BaseRefConfigStatus,
}

/// Type alias to represent some changes applied on a service.
pub(crate) type ChangesApplied = Vec<ChangeApplied>;

/// Represents a change applied on a service in an attempt to get closer to the
/// desired state.
pub(crate) struct ChangeApplied {
    pub change: DynChange,
    pub error: Option<String>,
}

/// Trait that defines some operations a Change implementation must support.
#[typetag::serde(tag = "type")]
pub(crate) trait Change: Debug {
    /// Format change to be used on a template.
    fn template_format(&self) -> Result<String>;
}

/// Type alias to represent a change trait object.
pub(crate) type DynChange = Box<dyn Change + Send + Sync>;

/// Status of the configuration from the base reference.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BaseRefConfigStatus {
    Valid,
    Invalid,
    Unknown,
}

impl BaseRefConfigStatus {
    /// Check if the configuration is invalid.
    pub(crate) fn is_invalid(&self) -> bool {
        *self == BaseRefConfigStatus::Invalid
    }
}