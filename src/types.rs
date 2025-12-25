use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// License status representing the current state of a license.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseStatus {
    Valid,
    Expired,
    Suspended,
    Revoked,
    Invalid,
    Trial,
    TrialExpired,
    NotActivated,
    #[serde(other)]
    Unknown,
}

impl Default for LicenseStatus {
    fn default() -> Self {
        Self::NotActivated
    }
}

/// License type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseType {
    Perpetual,
    Subscription,
    Trial,
}

impl Default for LicenseType {
    fn default() -> Self {
        Self::Perpetual
    }
}

/// A feature included in a license.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub key: String,
    pub name: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// License information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub id: String,
    pub key: String,
    pub status: LicenseStatus,
    #[serde(rename = "type")]
    pub license_type: LicenseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(default)]
    pub features: Vec<Feature>,
    #[serde(default)]
    pub max_activations: i32,
    #[serde(default)]
    pub current_activations: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_validated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl License {
    /// Check if a feature is enabled.
    pub fn has_feature(&self, feature_key: &str) -> bool {
        self.features
            .iter()
            .any(|f| f.key == feature_key && f.enabled)
    }

    /// Get a feature by key.
    pub fn get_feature(&self, feature_key: &str) -> Option<&Feature> {
        self.features.iter().find(|f| f.key == feature_key)
    }
}

/// An activation of a license on a machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activation {
    pub id: String,
    pub machine_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
}

/// Result of a license validation or activation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activations: Option<Vec<Activation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub cached: bool,
}

impl LicenseResult {
    pub fn success(license: License) -> Self {
        Self {
            valid: true,
            license: Some(license),
            activations: None,
            error: None,
            cached: false,
        }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            valid: false,
            license: None,
            activations: None,
            error: Some(error.into()),
            cached: false,
        }
    }
}

/// Result of starting a checkout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckoutResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl CheckoutResult {
    pub fn success(checkout_url: String, session_id: String) -> Self {
        Self {
            success: true,
            checkout_url: Some(checkout_url),
            session_id: Some(session_id),
            error: None,
        }
    }

    pub fn failure(error: impl Into<String>) -> Self {
        Self {
            success: false,
            checkout_url: None,
            session_id: None,
            error: Some(error.into()),
        }
    }
}

/// A product tier available for purchase.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductTier {
    pub id: String,
    pub slug: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub price: f64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[serde(default)]
    pub features: Vec<Feature>,
}
