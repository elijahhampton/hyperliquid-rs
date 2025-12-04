use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestingOrder {
    pub oid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderStatus {
    Resting { resting: RestingOrder },
    Error { error: String },
}

impl OrderStatus {
    pub fn is_resting(&self) -> bool {
        matches!(self, Self::Resting { .. })
    }

    pub fn oid(&self) -> Option<u64> {
        match self {
            Self::Resting { resting } => Some(resting.oid),
            Self::Error { .. } => None,
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Resting { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelStatus {
    Success(String),
    Error { error: String },
}

impl CancelStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(s) if s == "success")
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Success(_) => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningTwap {
    pub twap_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TwapOrderStatus {
    Running { running: RunningTwap },
    Error { error: String },
}

impl TwapOrderStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running { .. })
    }

    pub fn twap_id(&self) -> Option<u64> {
        match self {
            Self::Running { running } => Some(running.twap_id),
            Self::Error { .. } => None,
        }
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Running { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TwapCancelStatus {
    Success(String),
    Error { error: String },
}

impl TwapCancelStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(s) if s == "success")
    }

    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { error } => Some(error),
            Self::Success(_) => None,
        }
    }
}
